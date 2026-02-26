use bevy::prelude::*;
use crate::{AppState, GameState, config};
use crate::enemy::{Enemy};
use crate::enemy::{EnemyType};
use rand;
use std::collections::HashMap;
use crate::enemy;
use rand::seq::SliceRandom;


#[derive(Resource, Default)]
pub struct WaveStatus {
    pub is_running: bool,
    pub enemies_total: usize,
    pub enemies_remaining: usize,
    pub spawn_targets: HashMap<EnemyType, usize>,
    pub spawned_by_type: HashMap<EnemyType, usize>,
    pub spawn_timer: Timer,
    pub spawn_queue: Vec<EnemyType>,
}

pub fn start_wave(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut wave_status: ResMut<WaveStatus>,
    // ... other params
) {
    game_state.round += 1;
    let wave = (game_state.round as f32).powf(config::WAVE_EXPO).min(1.0) as usize;

    // 1. Calculate Targets
    let mut targets = HashMap::new();
    targets.insert(EnemyType::Normal, wave * config::WAVE_ENEMIES_PER_WAVE);
    
    if wave % config::WAVE_UNIQUE_ENEMY_WAVE == 0 { 
        targets.insert(EnemyType::Unique, ((wave as f32 / config::WAVE_UNIQUE_ENEMY_WAVE as f32) * 1.5) as usize); 
    }
    if wave % config::WAVE_LARGE_ENEMY_WAVE == 0 { 
        targets.insert(EnemyType::Large,wave / config::WAVE_LARGE_ENEMY_WAVE);
    }
    if wave % config::WAVE_COLOSSAL_ENEMY_WAVE == 0 { 
        targets.insert(EnemyType::Colossal, (wave / config::WAVE_COLOSSAL_ENEMY_WAVE / 2).min(1)); 
    }
    if wave % config::WAVE_BOSS_ENEMY_WAVE == 0 { 
        targets.insert(EnemyType::Boss, (wave / config::WAVE_BOSS_ENEMY_WAVE / 2).min(1)); 
    }

    let mut queue = Vec::new();
    for (etype, &count) in targets.iter() {
        for _ in 0..count {
            queue.push(*etype);
        }
    }

    let mut rng = rand::rng();
    queue.shuffle(&mut rng);

    let total = queue.len();
    wave_status.spawn_targets = targets;
    wave_status.spawn_queue = queue;
    wave_status.enemies_total = total;
    wave_status.enemies_remaining = total;
    wave_status.spawned_by_type.clear();
    wave_status.is_running = true;
    wave_status.spawn_timer = Timer::from_seconds(1.0, TimerMode::Repeating);

    info!("Wave {} started! Randomized queue ready with {} enemies.", wave, total);
}

pub fn spawn_tick_system(
    time: Res<Time>,
    mut wave_status: ResMut<WaveStatus>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !wave_status.is_running { return; }
    wave_status.spawn_timer.tick(time.delta());

    if wave_status.spawn_timer.just_finished() {
        let wave = game_state.round;

        let min_spawn = config::WAVE_ENEMIES_PER_FRAME[0] as f32;
        let max_spawn = config::WAVE_ENEMIES_PER_FRAME[1] as f32;
        let spawn_burst_limit = rand::random_range(min_spawn..max_spawn) as usize;

        for _ in 0..spawn_burst_limit {
            if let Some(etype) = wave_status.spawn_queue.pop() {
                let angle: f32 = rand::random_range(0.0..360.0);
                enemy::spawn_enemy(&mut commands, &mut meshes, &mut materials, angle, etype, &mut *game_state);
            } else { break; }
        }

        let min_ms = config::WAVE_ENEMIES_TIME_PER_FRAME[0] as f32;
        let max_ms = config::WAVE_ENEMIES_TIME_PER_FRAME[1] as f32;
        let random_ms = rand::random_range(min_ms..max_ms);
        wave_status.spawn_timer.set_duration(std::time::Duration::from_millis(random_ms as u64));
    }
}

pub fn check_wave_end(
    mut wave_status: ResMut<WaveStatus>,
    enemy_query: Query<&Enemy>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !wave_status.is_running { return; }
    let all_spawned = wave_status.spawn_queue.is_empty();
    let all_dead = enemy_query.is_empty();

    let still_to_spawn = wave_status.spawn_queue.len();
    wave_status.enemies_remaining = still_to_spawn + enemy_query.iter().count();

    if all_spawned && all_dead {
        wave_status.is_running = false;
        
        info!("Wave Complete! Total Spawned: {}. All enemies defeated.", wave_status.enemies_total);

        //Start Shop State
        next_state.set(AppState::Shop)
    }
}