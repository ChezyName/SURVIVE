use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::asset::RenderAssetUsages;
use crate::player::Player;
use bevy::mesh::Indices;
use crate::config;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
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
    mut enemy_query: Query<(&mut Transform, &Enemy), Without<Player>>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (mut enemy_transform, enemy_stats) in &mut enemy_query {
            let dir = (player_transform.translation - enemy_transform.translation).normalize_or_zero();
            enemy_transform.translation += dir * enemy_stats.speed * time.delta_secs();
            enemy_transform.rotation = Quat::from_rotation_z(dir.y.atan2(dir.x) - std::f32::consts::FRAC_PI_2);
        }
    }
}

fn enemy_collision_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (enemy_entity, enemy_transform, mut enemy_comp) in &mut enemy_query {
            let distance = player_transform.translation.distance(enemy_transform.translation);

            if distance < config::ENEMY_HIT_BOX {
                commands.entity(enemy_entity).despawn();
                
                println!("Player hit by Enemy with {} Damage", enemy_comp.damage);
            }
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    angle_degrees: f32, 
    sides: u32, 
) {
    //Polygon must be 3 sides min
    let sides = sides.max(3);

    let spawn_distance = config::ENEMY_SPAWN_DIST;
    let radians = angle_degrees.to_radians();
    let x = spawn_distance * radians.cos();
    let y = spawn_distance * radians.sin();
    let position = Vec3::new(x, y, 0.0);

    let angle_to_center = (Vec2::ZERO - position.xy()).to_angle();
    let rotation = Quat::from_rotation_z(angle_to_center - std::f32::consts::FRAC_PI_2);

    let mut verts = Vec::new();
    let shape_radius = 15.0;

    verts.push([0.0, 0.0, 0.0]); 

    for i in 0..sides {
        let angle = (std::f32::consts::TAU / sides as f32) * i as f32;
        verts.push([shape_radius * angle.cos(), shape_radius * angle.sin(), 0.0]);
    }

    let mut indices = Vec::new();
    for i in 1..sides {
        indices.extend_from_slice(&[0, i, i + 1]);
    }
    
    indices.extend_from_slice(&[0, sides, 1]);

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
            health: config::ENEMY_HEALTH,
            speed: config::ENEMY_SPEED,
            damage: config::ENEMY_HEALTH,
        },
    ));
}

pub fn take_damage(
    commands: &mut Commands,
    entity: Entity,
    enemy: &mut Enemy, // The component data
    damage: f32,
) {
    enemy.health -= damage;

    if enemy.health <= 0.0 {
        commands.entity(entity).despawn();
    }
}