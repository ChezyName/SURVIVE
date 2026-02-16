use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use crate::{config, enemy, GameState};
use crate::enemy::Enemy;

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct Lifetime(pub Timer);

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_projectiles, projectile_lifetime, projectile_collision));
    }
}

/// The "Factory" function: Call this from the player system.
pub fn spawn_projectile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player_transform: Transform,
    damage: f32,
    speed: f32,
) {
    // 1. Calculate dynamic rectangle size (Tracer effect)
    let width = 2.0;
    let length = (speed / 100.0).max(6.0); 
    let half_w = width / 2.0;
    let half_l = length / 2.0;

    let points = vec![
        [-half_w, half_l, 0.0],  [half_w, half_l, 0.0],
        [half_w, -half_l, 0.0], [-half_w, -half_l, 0.0],
    ];
    let indices = vec![0, 2, 1, 0, 3, 2];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    mesh.insert_indices(Indices::U32(indices));

    // 2. Spawn with all necessary logic
    commands.spawn((
        Projectile { damage, speed },
        Lifetime(Timer::from_seconds(config::BULLET_LIFETIME, TimerMode::Once)),
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(Color::WHITE)),
        player_transform, // Position and rotation inherited from player
    ));
}

fn move_projectiles(time: Res<Time>, mut query: Query<(&mut Transform, &Projectile)>) {
    for (mut transform, stats) in &mut query {
        let direction = transform.up(); 
        transform.translation += direction * stats.speed * time.delta_secs();
    }
}

fn projectile_lifetime(time: Res<Time>, mut commands: Commands, mut query: Query<(Entity, &mut Lifetime)>) {
    for (entity, mut lifetime) in &mut query {
        lifetime.0.tick(time.delta());
        if lifetime.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn projectile_collision(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &Projectile)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    mut game_state: ResMut<GameState>,
) {
    for (projectile_entity, projectile_transform, projectile) in &projectile_query {
        for (enemy_entity, enemy_transform, mut enemy_comp) in &mut enemy_query {
            let distance = projectile_transform.translation.distance(enemy_transform.translation);
            if distance < (config::BULLET_HIT_BOX + enemy_comp.size) {
                commands.entity(projectile_entity).despawn();

                enemy::take_damage(&mut commands, &mut *game_state, enemy_entity, &mut enemy_comp, projectile.damage);
                break;
            }
        }
    }
}