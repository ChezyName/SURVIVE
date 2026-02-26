use std::time::Duration;

use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use crate::items::{explosion, switch};
use crate::player::Player;
use crate::{config, enemy, GameState};
use crate::enemy::Enemy;
use crate::AppState;

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub hits: usize,
    pub max_hits: usize,
    pub half_w: f32,
    pub half_l: f32,
    pub homing: Option<Entity>,
}

#[derive(Component)]
pub struct Lifetime(pub Timer);

#[derive(Component)]
pub struct ExplosionVfx {
    pub timer: Timer,
    pub max_radius: f32,
    pub damage: f32,
    pub damaged_enemies: Vec<Entity>,
}

#[derive(Resource, Default)]
pub struct HomingSpawnQueue(pub Vec<HomingSpawnEvent>);

#[derive(Clone)]
pub struct HomingSpawnEvent {
    pub player_transform: Transform,
    pub target_entity: Entity,
    pub base_damage: f32,
    pub projectile_index: usize,
    pub total_projectiles: usize,
    pub delay: f32,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_projectiles, projectile_lifetime, projectile_collision, update_explosions, spawn_homing_projectiles).run_if(in_state(AppState::InGame)));
    }
}

/// The "Factory" function: Call this from the player system.
pub fn spawn_projectile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player: &mut Player,
    player_transform: Transform,
    homing: Option<Entity>,
) {
    //bulelt stats from player obj
    let size = (player.bullet_size / 100.0) * 1.0;
    let speed = player.bullet_speed;
    let damage = player.damage;
    let max_hits = player.bullet_pierce;
    let hits = 0;

    //default shape and size
    let width = size;
    let length = ((speed / 100.0).max(12.0) * size) / 2.0; 
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

    commands.spawn((
        Projectile { damage, speed, hits, max_hits, half_w, half_l, homing },
        Lifetime(Timer::from_seconds(config::BULLET_LIFETIME, TimerMode::Once)),
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(Color::WHITE)),
        player_transform,
    ));
}

fn move_projectiles(
    time: Res<Time>,
    mut projectile_query: Query<(&mut Transform, &Projectile)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Projectile>)>,
) {
    for (mut transform, projectile) in &mut projectile_query {
        let direction = if let Some(target) = projectile.homing {
            if let Ok(enemy_transform) = enemy_query.get(target) {
                let to_target_vec = enemy_transform.translation.truncate() - transform.translation.truncate();
                let dist = to_target_vec.length();
                let to_target = to_target_vec.normalize_or_zero();
                let current_dir = transform.up().truncate();

                let t = 1.0 - ((dist - switch::MIN_MAX_DIST[0]) / (switch::MIN_MAX_DIST[1] - switch::MIN_MAX_DIST[0])).clamp(0.0, 1.0);
                let turn_speed = switch::TURN_SPEED[0] + (switch::TURN_SPEED[1] - switch::TURN_SPEED[0]) * t;

                let new_dir = current_dir.lerp(to_target, turn_speed * time.delta_secs()).normalize_or_zero();
                let angle = new_dir.y.atan2(new_dir.x) - std::f32::consts::FRAC_PI_2;
                transform.rotation = Quat::from_rotation_z(angle);
                new_dir.extend(0.0)
            } else {
                transform.up().as_vec3()
            }
        } else {
            transform.up().as_vec3()
        };

        transform.translation += direction * projectile.speed * time.delta_secs();
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

fn rect_circle_intersect(
    rect_transform: &Transform,
    half_w: f32,
    half_l: f32,
    circle_pos: Vec2,
    circle_radius: f32,
) -> bool {
    // transform circle center into the rectangle's local space
    let rect_pos = rect_transform.translation.truncate();
    let rect_angle = rect_transform.rotation.to_euler(EulerRot::XYZ).2;

    let cos = rect_angle.cos();
    let sin = rect_angle.sin();

    let delta = circle_pos - rect_pos;

    // rotate delta into rect local space
    let local_x = delta.x * cos + delta.y * sin;
    let local_y = -delta.x * sin + delta.y * cos;

    // clamp to nearest point on rect
    let clamped_x = local_x.clamp(-half_w, half_w);
    let clamped_y = local_y.clamp(-half_l, half_l);

    // distance from circle center to nearest point on rect
    let dist_x = local_x - clamped_x;
    let dist_y = local_y - clamped_y;

    (dist_x * dist_x + dist_y * dist_y) < (circle_radius * circle_radius)
}

fn projectile_collision(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Transform, &mut Projectile)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    mut player_query: Query<(&mut Player, &Transform)>,
    mut game_state: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut homing_queue: ResMut<HomingSpawnQueue>,
) {
    if let Ok((mut player, player_transform)) = player_query.single_mut() {
        for (projectile_entity, projectile_transform, mut projectile) in &mut projectile_query {
            for (enemy_entity, enemy_transform, mut enemy_comp) in &mut enemy_query {
                let enemy_pos = enemy_transform.translation.truncate();
                if rect_circle_intersect(
                    projectile_transform,
                    projectile.half_w,
                    projectile.half_l,
                    enemy_pos,
                    enemy_comp.size,
                ) {
                    projectile.hits += 1;
                    if projectile.hits >= projectile.max_hits {
                        commands.entity(projectile_entity).despawn();
                    }

                    let mut damage = projectile.damage;
                    if player.crit_percent > 0.0 && rand::random_bool((player.crit_percent / 100.0).clamp(0.0, 1.0) as f64) {
                        damage += damage * 0.75;
                    }

                    enemy::take_damage(&mut commands, &mut *game_state, &mut *player, enemy_entity, &mut enemy_comp, damage);

                    if player.missiles > 0 && projectile.homing.is_none() {
                        let total_delay = Duration::from_millis(switch::SPAWN_TIME_MS).as_secs_f32();
                        let delay_per = if player.missiles <= 1 {
                            0.0
                        } else {
                            total_delay / (player.missiles - 1) as f32
                        };

                        for i in 0..player.missiles {
                            homing_queue.0.push(HomingSpawnEvent {
                                player_transform: *player_transform,
                                target_entity: enemy_entity,
                                base_damage: projectile.damage * (switch::DAMAGE_REDUCTION_PER_BULLET / 100.0).clamp(0.0, 1.0),
                                projectile_index: i,
                                total_projectiles: player.missiles,
                                delay: i as f32 * delay_per,
                            });
                        }
                    }

                    // Explosion
                    if player.explosive_bullets {
                        let explosion_pos = projectile_transform.translation.truncate();
                        let explosion_radius = config::EXPLOSION_RADIUS;
                        let explosion_damage = damage / 2.0;

                        // Damage nearby enemies
                        for (other_entity, other_transform, mut other_enemy) in &mut enemy_query {
                            if other_entity == enemy_entity { continue; } // skip if hit
                            let dist = other_transform.translation.truncate().distance(explosion_pos);
                            if dist < explosion_radius {
                                let falloff = 1.0 - (dist / explosion_radius); // 1.0 at center, 0.0 at edge
                                let exp_damage = explosion_damage * falloff;
                                enemy::take_damage(&mut commands, &mut *game_state, &mut *player, other_entity, &mut other_enemy, exp_damage);

                                if player.missiles > 0 && projectile.homing.is_none() && player.missile_explosion {
                                    let total_delay = Duration::from_millis(switch::SPAWN_TIME_MS).as_secs_f32();
                                    let delay_per = if player.missiles <= 1 {
                                        0.0
                                    } else {
                                        total_delay / (player.missiles - 1) as f32
                                    };

                                    for i in 0..player.missiles {
                                        homing_queue.0.push(HomingSpawnEvent {
                                            player_transform: *player_transform,
                                            target_entity: enemy_entity,
                                            base_damage: projectile.damage * (switch::DAMAGE_REDUCTION_PER_BULLET / 100.0).clamp(0.0, 1.0),
                                            projectile_index: i,
                                            total_projectiles: player.missiles,
                                            delay: i as f32 * delay_per,
                                        });
                                    }
                                }
                            }
                        }

                        commands.spawn((
                            Mesh2d(meshes.add(Circle::new(1.0))),
                            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(1.0, 0.5, 0.1, 0.6)))),
                            Transform::from_translation(explosion_pos.extend(1.0)),
                            ExplosionVfx {
                                timer: Timer::from_seconds(0.15, TimerMode::Once),
                                max_radius: config::EXPLOSION_RADIUS * player.explosive_radius,
                                damage: damage * explosion::EXPLOSION_DAMAGE_MULTI,
                                damaged_enemies: vec![enemy_entity], // pre-add the directly hit enemy
                            },
                        ));
                    }

                    break;
                }
            }
        }
    }
}

pub fn update_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
    mut homing_queue: ResMut<HomingSpawnQueue>,
    mut vfx_query: Query<(Entity, &mut Transform, &MeshMaterial2d<ColorMaterial>, &mut ExplosionVfx)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), Without<ExplosionVfx>>,
    mut player_query: Query<(&mut Player, &Transform), (Without<ExplosionVfx>, Without<Enemy>)>,
) {
    // Pass 1: VFX update, collect explosion data
    let mut active_explosions: Vec<(Entity, Vec2, f32, f32, Vec<Entity>)> = Vec::new();
    for (entity, mut transform, material_handle, mut vfx) in &mut vfx_query {
        vfx.timer.tick(time.delta());
        let progress = vfx.timer.fraction();
        let radius = vfx.max_radius * progress;
        transform.scale = Vec3::splat(radius);
        if let Some(mat) = materials.get_mut(&material_handle.0) {
            mat.color = Color::srgba(1.0, 0.5 - progress * 0.3, 0.1, 0.6 * (1.0 - progress));
        }
        active_explosions.push((entity, transform.translation.truncate(), radius, vfx.damage, vfx.damaged_enemies.clone()));
    }

    // Pass 2: collect hits — (vfx_entity, enemy_entity, damage)
    let mut hits: Vec<(Entity, Entity, f32)> = Vec::new();
    for (vfx_entity, explosion_pos, radius, damage, already_damaged) in &active_explosions {
        for (enemy_entity, enemy_transform, _) in enemy_query.iter() {
            if already_damaged.contains(&enemy_entity) { continue; }
            let dist = enemy_transform.translation.truncate().distance(*explosion_pos);
            if dist < *radius {
                let falloff = 1.0 - (dist / *radius);
                hits.push((*vfx_entity, enemy_entity, damage * falloff));
            }
        }
    }

    // Pass 3: collect player info then drop — no borrow held
    let player_info = player_query.single_mut().ok().map(|(p, t)| {
        (p.missiles, p.missile_explosion, *t)
    });

    // Pass 4: apply damage — safe now, no overlapping borrows
    let mut newly_damaged: std::collections::HashMap<Entity, Vec<Entity>> = std::collections::HashMap::new();
    for (vfx_entity, enemy_entity, hit_damage) in hits {
        if let Ok(mut enemy_comp) = enemy_query.get_mut(enemy_entity).map(|(_, _, e)| e) {
            if let Ok((mut player, _)) = player_query.single_mut() {
                enemy::take_damage(&mut commands, &mut *game_state, &mut *player, enemy_entity, &mut enemy_comp, hit_damage);
                newly_damaged.entry(vfx_entity).or_default().push(enemy_entity);

                if let Some((missiles, missile_explosion, p_transform)) = player_info {
                    if missiles > 0 && missile_explosion {
                        let total_delay = Duration::from_millis(switch::SPAWN_TIME_MS).as_secs_f32();
                        let delay_per = if missiles <= 1 { 0.0 } else { total_delay / (missiles - 1) as f32 };
                        for i in 0..missiles {
                            homing_queue.0.push(HomingSpawnEvent {
                                player_transform: p_transform,
                                target_entity: enemy_entity,
                                base_damage: (hit_damage.max(1.0) / explosion::EXPLOSION_DAMAGE_MULTI) * (switch::DAMAGE_REDUCTION_PER_BULLET / 100.0).clamp(0.0, 1.0),
                                projectile_index: i,
                                total_projectiles: missiles,
                                delay: i as f32 * delay_per,
                            });
                        }
                    }
                }
            }
        }
    }

    // Pass 5: write back damaged lists
    for (vfx_entity, enemies) in newly_damaged {
        if let Ok((_, _, _, mut vfx)) = vfx_query.get_mut(vfx_entity) {
            vfx.damaged_enemies.extend(enemies);
        }
    }

    // Pass 6: despawn finished
    let finished: Vec<Entity> = vfx_query
        .iter()
        .filter(|(_, _, _, vfx)| vfx.timer.is_finished())
        .map(|(e, _, _, _)| e)
        .collect();
    for entity in finished {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_homing_projectiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut queue: ResMut<HomingSpawnQueue>,
    mut player_query: Query<&mut Player>,
    enemy_query: Query<&Transform, With<Enemy>>,
    time: Res<Time>,
) {
    if queue.0.is_empty() { return; }
    let Ok(mut player) = player_query.single_mut() else { return };

    // Tick all delays
    for event in queue.0.iter_mut() {
        event.delay -= time.delta_secs();
    }

    let mut ready = Vec::new();
    queue.0.retain(|e| {
        if e.delay <= 0.0 {
            ready.push(e.clone());
            false
        } else {
            true
        }
    });

    for event in ready {
        let Ok(enemy_transform) = enemy_query.get(event.target_entity) else { continue };

        let angle_offset = if event.total_projectiles <= 1 {
            if event.projectile_index % 2 == 0 { 60.0_f32.to_radians() } else { -60.0_f32.to_radians() }
        } else {
            let t = event.projectile_index as f32 / (event.total_projectiles - 1) as f32;
            let spread = 120.0; // -60 to +60
            -spread / 2.0 + t * spread
        };

        // Aim from player toward enemy, offset by arc angle
        let to_enemy = (enemy_transform.translation.truncate() - event.player_transform.translation.truncate()).normalize_or_zero();
        let base_angle = to_enemy.y.atan2(to_enemy.x) - std::f32::consts::FRAC_PI_2;
        let mut spawn_transform = event.player_transform;
        spawn_transform.rotation = Quat::from_rotation_z(base_angle + angle_offset.to_radians());

        let size = ((player.bullet_size / 100.0) * (switch::SIZE_MULTI / 100.0) * 1.0).max(3.0);
        let speed = (player.bullet_speed * (switch::SPEED_MULTI / 100.0)).clamp(switch::SPEED_MIN, 999.99);
        let width = size;
        let length = ((speed / 100.0).max(12.0) * size) / 2.0;
        let half_w = width / 2.0;
        let half_l = length / 2.0;

        let points = vec![
            [-half_w, half_l, 0.0], [half_w, half_l, 0.0],
            [half_w, -half_l, 0.0], [-half_w, -half_l, 0.0],
        ];
        let indices = vec![0, 2, 1, 0, 3, 2];
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
        mesh.insert_indices(Indices::U32(indices));

        commands.spawn((
            Projectile {
                damage: event.base_damage * (switch::DAMAGE_REDUCTION_PER_BULLET / 100.0),
                speed,
                hits: 0,
                max_hits: 1,
                half_w,
                half_l,
                homing: Some(event.target_entity),
            },
            Lifetime(Timer::from_seconds(config::BULLET_LIFETIME, TimerMode::Once)),
            Mesh2d(meshes.add(mesh)),
            MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 0.0, 1.0))), // green tint for homing
            spawn_transform,
        ));
    }
}