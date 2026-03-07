//TODO: Add Wave Dist Card
use std::time::Duration;

use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use crate::enemy::Enemy;
use crate::items::switch;
use crate::player::Player;
use crate::projectile::{HomingSpawnEvent, HomingSpawnQueue};
use crate::{GameState, audio, enemy};
use crate::AppState;

#[derive(Component)]
pub struct Wave { pub damage: f32, pub speed: f32, pub max_radius: f32,pub current_radius: f32,pub damaged_enemies: Vec<Entity> }

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) { app.add_systems(Update, update_waves.run_if(in_state(AppState::InGame))).add_systems(OnEnter(AppState::Shop), despawn_waves); }
}

fn despawn_waves(mut commands: Commands, query: Query<Entity, With<Wave>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_wave(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    location: Vec3,
    damage: f32,
    max_radius: f32,
    speed: f32,
) {
    commands.spawn((
        Wave {
            damage,
            speed,
            max_radius,
            current_radius: 0.0,
            damaged_enemies: Vec::new(),
        },
        Mesh2d(meshes.add(annulus_mesh(0.0, 4.0))), // thin ring
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(1.0, 1.0, 1.0, 0.25)))),
        Transform::from_translation(location),
    ));
}

fn annulus_mesh(inner: f32, outer: f32) -> Mesh {
    let segments = 64;
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    for i in 0..segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let cos = angle.cos();
        let sin = angle.sin();
        positions.push([cos * inner, sin * inner, 0.0]);
        positions.push([cos * outer, sin * outer, 0.0]);
    }

    for i in 0..segments {
        let a = (i * 2) as u32;
        let b = (i * 2 + 1) as u32;
        let c = ((i * 2 + 2) % (segments * 2)) as u32;
        let d = ((i * 2 + 3) % (segments * 2)) as u32;
        indices.extend_from_slice(&[a, b, c, b, d, c]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn update_waves(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    mut wave_query: Query<(Entity, &mut Transform, &mut Wave, &mut Mesh2d)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), Without<Wave>>,
    mut player_query: Query<&mut Player, Without<Wave>>,
    mut game_state: ResMut<GameState>,
    mut homing_queue: ResMut<HomingSpawnQueue>,
    mut sounds: ResMut<audio::GlobalSounds>,
) {
    for (wave_entity, wave_transform, mut wave, mut mesh2d) in &mut wave_query {
        wave.current_radius += wave.speed * time.delta_secs();

        // Update mesh to new radius
        let thickness = 6.0;
        let inner = (wave.current_radius - thickness).max(0.0);
        let outer = wave.current_radius;
        mesh2d.0 = meshes.add(annulus_mesh(inner, outer));

        // Fade out as it reaches max
        let progress = (wave.current_radius / wave.max_radius).clamp(0.0, 1.0);
        let alpha = 0.7 * (1.0 - progress);

        // Check enemies inside the ring
        let wave_pos = wave_transform.translation.truncate();
        if let Ok(mut player) = player_query.single_mut() {
            for (enemy_entity, enemy_transform, mut enemy_comp) in &mut enemy_query {
                if wave.damaged_enemies.contains(&enemy_entity) { continue; }

                let dist = enemy_transform.translation.truncate().distance(wave_pos);
                if dist <= wave.current_radius {
                    enemy::take_damage(&mut commands, &mut *game_state, &mut *player, enemy_entity, &mut enemy_comp, wave.damage, &mut sounds);
                    wave.damaged_enemies.push(enemy_entity);

                    if player.missiles > 0 && player.wave_missile {
                        let total_delay = Duration::from_millis(switch::SPAWN_TIME_MS).as_secs_f32();
                        let delay_per = if player.missiles <= 1 {
                            0.0
                        } else {
                            total_delay / (player.missiles - 1) as f32
                        };

                        for i in 0..player.missiles {
                            homing_queue.0.push(HomingSpawnEvent {
                                player_transform: *wave_transform,
                                target_entity: enemy_entity,
                                base_damage: wave.damage * (switch::DAMAGE_REDUCTION_PER_BULLET / 100.0).clamp(0.0, 1.0),
                                projectile_index: i,
                                total_projectiles: player.missiles,
                                delay: i as f32 * delay_per,
                            });
                        }
                    }
                }
            }
        }

        if wave.current_radius >= wave.max_radius {
            commands.entity(wave_entity).despawn();
        }
    }
}