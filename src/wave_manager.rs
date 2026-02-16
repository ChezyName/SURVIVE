use bevy::prelude::*;
use crate::{GameState, config};
use crate::enemy::{Enemy};
use crate::enemies::{EnemyType};
use rand;
use std::collections::HashMap;
use crate::enemy;


#[derive(Resource, Default)]
pub struct WaveStatus {
    pub is_running: bool,
    pub enemies_total: usize,
    pub enemies_spawned: usize,
    pub spawn_targets: HashMap<EnemyType, usize>,
    pub spawned_by_type: HashMap<EnemyType, usize>,
    pub spawn_timer: Timer,
}

pub fn start_wave(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut wave_status: ResMut<WaveStatus>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    game_state.round += 1;
    wave_status.is_running = true;

    let wave = game_state.round;

    let mut targets = HashMap::new();
    targets.insert(EnemyType::Normal, wave * config::WAVE_ENEMIES_PER_WAVE);
    if (wave % config::WAVE_LARGE_ENEMY_WAVE == 0) { 
        targets.insert(EnemyType::Large, wave / config::WAVE_LARGE_ENEMY_WAVE); 
    }

    if (wave % config::WAVE_BOSS_ENEMY_WAVE == 0) { 
        targets.insert(EnemyType::Boss, wave / config::WAVE_BOSS_ENEMY_WAVE); 
    }

    let total: usize = targets.values().sum();

    wave_status.spawn_targets = targets;
    wave_status.enemies_total = total as usize;
    wave_status.enemies_spawned = 0;
    wave_status.spawned_by_type.clear();
    wave_status.is_running = true;
    wave_status.spawn_timer = Timer::from_seconds(1.0, TimerMode::Repeating);

    info!("Starting Wave {}! Spawning {} enemies.", game_state.round, total);
}

pub fn spawn_tick_system(
    time: Res<Time>,
    mut wave_status: ResMut<WaveStatus>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    enemy_query: Query<&Enemy>,
) {
    if !wave_status.is_running { return; }

    wave_status.spawn_timer.tick(time.delta());

    if wave_status.spawn_timer.just_finished() {
        let current_on_screen = enemy_query.iter().count();
        
        if current_on_screen < config::WAVE_MAX_ENEMIES_PER_FRAME && 
           wave_status.enemies_spawned < wave_status.enemies_total 
        {
            let type_to_spawn = wave_status.spawn_targets.iter().find_map(|(etype, &target)| {
                let current_spawned = wave_status.spawned_by_type.get(etype).unwrap_or(&0);
                if *current_spawned < target {
                    Some(*etype)
                } else {
                    None
                }
            });

            if let Some(etype) = type_to_spawn {
                let angle: f32 = rand::random_range(0.0..360.0);

                enemy::spawn_enemy(
                    &mut commands, 
                    &mut meshes, 
                    &mut materials, 
                    angle, 
                    etype
                );

                wave_status.enemies_spawned += 1;
                *wave_status.spawned_by_type.entry(etype).or_insert(0) += 1;
                
                info!("Spawned {:?}. Total: {}/{}", etype, wave_status.enemies_spawned, wave_status.enemies_total);
            }
        }
    }
}

pub fn check_wave_end(
    mut wave_status: ResMut<WaveStatus>,
    enemy_query: Query<&Enemy>,
) {
    if !wave_status.is_running { return; }
    let all_spawned = wave_status.enemies_spawned >= wave_status.enemies_total;
    let all_dead = enemy_query.is_empty();

    if all_spawned && all_dead {
        wave_status.is_running = false;
        
        info!("Wave Complete! Total Spawned: {}. All enemies defeated.", wave_status.enemies_spawned);

        //Start Shop State
    }
}