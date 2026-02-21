use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::asset::RenderAssetUsages;
use crate::AppState;
use crate::player::Player;
use bevy::mesh::Indices;
use crate::config;
use crate::player;
use crate::GameState;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
    pub price_tag: usize,
    pub size: f32,
    pub movement: MovementType,
    pub is_switch: bool,
    pub rand_bool: bool,
    pub switch_timer: Timer,
    pub switch_time_range: [f32; 2],
    pub start_dist: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MovementType {
    Line,
    Circular,   //Curves Towards The Player
    ZigZag,     //Goes in Zigs
    Switch,     //Swaps Between Multiple Movement Modes
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EnemyType {
    Normal,     //Regular Line Enemies
    Unique,     //Smaller Fast / Movement Enemies
    Large,      //Larger Tanky Enemies
    Colossal,   //Massive Tanky Enemies
    Boss,       //Boss Enemeis with Unique Properties
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (enemy_ai_system, enemy_collision_system));
    }
}

fn enemy_ai_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy), Without<Player>>,
) {
    if let Ok(player_transform) = player_query.single() {
        let seconds = time.elapsed_secs();

        for (mut enemy_transform, mut enemy_stats) in enemy_query.iter_mut() {
            enemy_stats.switch_timer.tick(time.delta());

            let to_player = (player_transform.translation - enemy_transform.translation).normalize_or_zero();

            if enemy_stats.movement == MovementType::Switch || (enemy_stats.is_switch && enemy_stats.switch_timer.is_finished()) {
                //switch movement type
                let new_movement = match rand::random_range(0..3) {
                    0 => MovementType::Line,
                    1 => MovementType::Circular,
                    2 => MovementType::ZigZag,
                    _ => MovementType::Line, //defaults to line
                };

                enemy_stats.movement = new_movement;
                let random_ms = rand::random_range(enemy_stats.switch_time_range[0]..enemy_stats.switch_time_range[1]);
                enemy_stats.switch_timer.set_duration(std::time::Duration::from_millis(random_ms as u64));
            }
            
            let move_dir = match enemy_stats.movement {
                MovementType::Line => to_player,

                MovementType::Circular => {
                    let distance = enemy_transform.translation.distance(player_transform.translation);
                    let t = (distance / enemy_stats.start_dist).clamp(0.0, 1.0);
                    let curve_strength = config::lerp(0.5, 1.0, t); 

                    let offset_angle = seconds.sin() * curve_strength;
                    let modifier: i32 = if enemy_stats.rand_bool { -1 } else { 1 };
                    Quat::from_rotation_z(modifier as f32 * offset_angle) * to_player
                }

                MovementType::ZigZag => {
                    let perpendicular = Vec3::new(-to_player.y, to_player.x, 0.0);
                    let zig_frequency = 2.0;
                    let zig_amplitude = 5.0;
                    let side_step = perpendicular * (seconds * zig_frequency).sin() * zig_amplitude;
                    (to_player + side_step).normalize()
                }

                MovementType::Switch => { to_player } //default to line if zig-zag
            };

            enemy_transform.translation += move_dir * enemy_stats.speed * time.delta_secs();

            if move_dir != Vec3::ZERO {
                let angle = move_dir.y.atan2(move_dir.x);
                enemy_transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
            }
        }
    }
}

fn enemy_collision_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &mut Player)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (player_entity, player_transform, mut player_comp) in &mut player_query {
        for (enemy_entity, enemy_transform, mut enemy_comp) in &mut enemy_query {
            let distance = player_transform.translation.distance(enemy_transform.translation);

            if distance < (config::PLAYER_HIT_BOX + enemy_comp.size) {
                commands.entity(enemy_entity).despawn();
                player::take_damage(&mut commands, player_entity, &mut player_comp, enemy_comp.damage, &mut next_state);
            }
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    angle_degrees: f32, 
    enemy_type: EnemyType,
) {
    let enemy_data = enemy_type.get_config();
    let sides = enemy_data.sides.max(3); //polygons are 3 sides min

    let spawn_distance = rand::random_range(config::ENEMY_SPAWN_DIST[0]..config::ENEMY_SPAWN_DIST[1]);
    let radians = angle_degrees.to_radians();
    let x = spawn_distance * radians.cos();
    let y = spawn_distance * radians.sin();
    let position = Vec3::new(x, y, 0.0);

    let direction_to_center = Vec2::ZERO - position.xy();
    let angle = direction_to_center.y.atan2(direction_to_center.x);
    let rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);

    let mut verts = Vec::new();
    let shape_radius = enemy_data.size;

    verts.push([0.0, 0.0, 0.0]); 

    for i in 0..sides {
        let start_offset = if sides == 3 { 
            std::f32::consts::FRAC_PI_2 //Triangle Fix
        } else if sides == 4 { 
            std::f32::consts::FRAC_PI_4 //45deg for Squares
        } else { 0.0 };

        let angle = start_offset + (std::f32::consts::TAU / sides as f32) * i as f32;
        verts.push([shape_radius * angle.cos(), shape_radius * angle.sin(), 0.0]);
    }

    let mut indices: Vec<u32> = Vec::new();
    for i in 1..sides {
        indices.extend_from_slice(&[0, i as u32, i as u32 + 1]);
    }
    
    indices.extend_from_slice(&[0, sides as u32, 1]);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
    mesh.insert_indices(Indices::U32(indices));

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.1, 0.1))),
        Transform {
            translation: position,
            rotation,
            ..default()
        },
        Enemy {
            health: enemy_data.health,
            speed: enemy_data.speed,
            damage: enemy_data.damage,
            price_tag: enemy_data.reward,
            movement: enemy_data.movement,
            size: enemy_data.size,
            is_switch: enemy_data.movement == MovementType::Switch,
            switch_timer: Timer::from_seconds(config::lerp(config::ENEMY_SWITCH_TIME_RANGE[0], config::ENEMY_SWITCH_TIME_RANGE[1], rand::random_range(0.0..1.0)), TimerMode::Repeating),
            switch_time_range: config::ENEMY_SWITCH_TIME_RANGE,
            rand_bool: rand::random_bool(0.5),
            start_dist: spawn_distance,
        },
    ));
}

pub fn take_damage(
    commands: &mut Commands,
    game_state: &mut GameState,
    player: &mut Player,
    entity: Entity,
    enemy: &mut Enemy,
    damage: f32
) {
    if enemy.health <= 0.0 { return; }

    let clamped_damage = damage.min(enemy.health);
    enemy.health -= damage;

    if enemy.health <= 0.0 {
        commands.entity(entity).despawn();
        game_state.money += enemy.price_tag * 5000;
        if player.life_steal > 0.0 {
            player.health = (player.health + ((player.life_steal / 100.0) * clamped_damage)).clamp(0.0, player.max_health)
        }
        
    }
}