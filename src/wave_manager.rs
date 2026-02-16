use bevy::prelude::*;
use crate::GameState;
use crate::enemy::{self, Enemy};
use rand::prelude::*;

#[derive(Resource, Default)]
pub struct WaveStatus {
    pub is_running: bool,
    pub enemy_count: usize,
}

pub fn start_wave(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut wave_status: ResMut<WaveStatus>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();
    game_state.round += 1;
    wave_status.is_running = true;

    let spawn_amount = 5 + (game_state.round * 2);
    wave_status.enemy_count = spawn_amount as usize;

    info!("Starting Wave {}! Spawning {} enemies.", game_state.round, spawn_amount);

    for i in 0..spawn_amount {
        let random_deg: f32 = rng.random_range(0.0..=360.0);
        enemy::spawn_enemy(&mut commands, &mut meshes, &mut materials, random_deg, 3);
    }
}

pub fn check_wave_end(
    mut wave_status: ResMut<WaveStatus>,
    enemy_query: Query<&Enemy>,
) {
    if !wave_status.is_running { return; }

    if enemy_query.is_empty() {
        wave_status.is_running = false;
        wave_status.enemy_count = 0;
        
        info!("Wave Complete! All enemies defeated.");
        //Start Shop Mode
    }
}