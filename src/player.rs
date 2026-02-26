use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use crate::AppState;
use crate::projectile;
use crate::config;
use crate::wave;

//Not a good way to hold items or stats, should be module but...
#[derive(Component)]
pub struct Player {
    pub fire_timer: Timer,
    pub fire_rate: f32,
    pub damage: f32,
    pub bullet_speed: f32,
    pub pellets: usize,
    pub bullet_spread: f32,
    pub health: f32,
    pub max_health: f32,
    pub life_steal: f32,
    pub bullet_size: f32,
    pub bullet_pierce: usize,
    pub crit_percent: f32,
    pub has_lifeline: bool,
    pub explosive_bullets: bool,
    pub explosive_radius: f32,
    pub missiles: usize,
    pub missile_explosion: bool,
    pub gold_per_second: f32,
    pub gold_multi: f32,
    pub wave_timer: Timer, //pulsating waves that deal damage
    pub wave_time: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            fire_timer: Timer::from_seconds(60.0/config::PLAYER_FIRE_RATE, TimerMode::Once),
            fire_rate: config::PLAYER_FIRE_RATE,
            damage: config::BULLET_DAMAGE,
            bullet_speed: config::BULLET_SPEED,
            pellets: 1,
            bullet_spread: 0.0,
            health: config::PLAYER_MAX_HEALTH,
            max_health: config::PLAYER_MAX_HEALTH,
            life_steal: 0.0,
            bullet_size: 100.0,
            bullet_pierce: 1, //can only hit one target before death
            crit_percent: 0.0, //crit in % (0 - 100)%
            has_lifeline: false,
            explosive_bullets: false,
            explosive_radius: 1.0,
            missiles: 0,
            missile_explosion: false,
            gold_per_second: 0.0,
            gold_multi: 1.0,
            wave_timer: Timer::from_seconds(1.0, TimerMode::Once),
            wave_time: -1.0,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (player_aim_system, player_shooting, player_wave_update).run_if(in_state(AppState::InGame)));
    }
}

fn player_wave_update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut time: ResMut<Time<Virtual>>,
) {
    if time.is_paused() { return; }

    if let Ok((transform, mut player)) = player_query.single_mut() {
        player.wave_timer.tick(time.delta());

        if player.wave_time != -1.0 && player.wave_timer.is_finished() {
            //spawn wave
            wave::spawn_wave(
                &mut commands,
                &mut meshes,
                &mut materials,
                transform.translation,
                player.damage / 4.0,
                500.0,
                player.bullet_speed * 0.75,
            );

            player.wave_timer.reset();
        }
    }
}

fn get_hexagon(radius: f32)-> Vec<[f32; 3]> {
    let mut verts = Vec::with_capacity(7);

    for i in 1..=6 {
        let angle = (PI / 3.0) * i as f32;

        let x = radius * angle.cos();
        let y = radius * angle.sin();

        verts.push([x, y, 0.0]);
    }

    verts.push(verts[0]);
    return verts;
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let radius = 15.0;
    let barel_length = radius / 2.5;

    // draw a Hexagon for player
    // with a square point for the tip
    let mut verts = get_hexagon(radius);

    //get points 0 - 1, move up & dupe 4x
    let top_left = [verts[0][0] - (radius/3.0), verts[0][1] + barel_length, 0.0];
    let top_right = [verts[1][0] + (radius/3.0), verts[1][1] + barel_length, 0.0];

    let bottom_left = [verts[0][0] - (radius/3.0), verts[0][1] + 0.0, 0.0];
    let bottom_right = [verts[1][0] + (radius/3.0), verts[1][1] + 0.0, 0.0];

    verts.extend([
        bottom_left,
        top_left,
        top_right,
        bottom_right,
    ]);

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    
    // Insert the coordinates
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);

    // 6 points for hex
    // 4 points for trig piece / shooter tip
    // 10 total points, 9 inc zero
    let indices = vec![
        //Hexagon
        0, 1, 2,
        0, 2, 3,
        0, 3, 4,
        0, 5, 6,
        4, 5, 6,

        //Tip
        7, 8, 9,
        9, 10, 7,

    ];

    mesh.insert_indices(Indices::U32(indices));

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player::default(),
    ));
}

fn player_aim_system(
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut time: ResMut<Time<Virtual>>,
) {
    if time.is_paused() { return; }

    let (camera, camera_transform) = camera_query.into_inner();
    for mut player_transform in &mut player_query {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                let direction: Vec2 = world_pos - player_transform.translation.xy();
                player_transform.rotation = Quat::from_rotation_z(direction.to_angle() - FRAC_PI_2);
            }
        }
    }
}

fn player_shooting(
    mut commands: Commands,
    time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<(&Transform, &mut Player)>,
) {
    if let Ok((transform, mut player)) = player_query.single_mut() {
        player.fire_timer.tick(time.delta());

        if mouse_input.pressed(MouseButton::Left) && player.fire_timer.is_finished() {
            player.fire_timer.reset();

            for i in 0..player.pellets {
                let mut b_transform = *transform;
                let spread = player.bullet_spread;

                let angle = if player.pellets <= 1 {
                    if spread > 0.0 {
                        rand::random_range(-spread..spread)
                    } else {
                        0.0
                    }
                } else {
                    let t = i as f32 / (player.pellets - 1) as f32;
                    -spread + t * (spread * 2.0)
                };

                if spread > 0.0 {
                    b_transform.rotate_z(angle.to_radians());
                }

                projectile::spawn_projectile(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut *player,
                    b_transform,
                    None,
                );
            }
        }
    }
}

pub fn take_damage(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    entity: Entity,
    player: &mut Player, // self
    damage: f32,
    next_state: &mut ResMut<NextState<AppState>>,
) {
    player.health -= damage;

    if player.health <= 0.0 {
        if player.has_lifeline {
            player.has_lifeline = false;
            player.health = player.max_health;

            wave::spawn_wave(
                commands,
                meshes,
                materials,
                vec3(0.0, 0.0, 0.0),
                player.damage.max(player.max_health),
                5000.0,
                player.bullet_speed.max(250.0),
            );

            return;
        }

        info!("Player has died");
        next_state.set(AppState::GameOver);
    }
}