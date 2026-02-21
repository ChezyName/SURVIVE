use bevy::prelude::*;
use crate::enemy::{EnemyType, MovementType};
use rand::{self, seq::IndexedRandom, seq::SliceRandom};

//Shop Defaults
pub const REROLL_COST_PER_REROLL: usize = 10; //Cost of Re-Roll Per Re-Roll

//Player Defaults
pub const PLAYER_MAX_HEALTH: f32 = 100.0;
pub const PLAYER_FIRE_RATE: f32 = 200.0; // 60 / rate where rate = RPM (rounds per min)
pub const PLAYER_HIT_BOX: f32 = 20.0; 

//UI Defaults
pub const UI_HEALTH_DIV: f32 = 10.0; //How much health per segment

//Bullet Defaults
pub const BULLET_SPEED: f32 = 250.0;
pub const BULLET_LIFETIME: f32 = 15.0;
pub const BULLET_DAMAGE: f32 = 15.0;
pub const BULLET_HIT_BOX: f32 = 2.0; 

//Basic Enemy Stuff, Multiplies Later
pub const ENEMY_SPAWN_DIST: f32 = 800.0;
pub const ENEMY_SWITCH_TIME_RANGE: [f32; 2] = [150.0, 1500.0]; //time range in ms

//Wave Spawning
pub const WAVE_ENEMIES_PER_FRAME: [i32; 2] = [5, 80]; //How Many Enemies (Min - Max) Can Spawn Per Frame
pub const WAVE_ENEMIES_TIME_PER_FRAME: [i32; 2] = [50, 1000]; //How Much Time Between The Frames (Min - Max) in ms
pub const WAVE_ENEMIES_PER_WAVE: usize = 6; //How Many Enemies Per Wave
pub const WAVE_UNIQUE_ENEMY_WAVE: usize = 2; //Spawn Large Enemies Every 2 (Other) Waves
pub const WAVE_LARGE_ENEMY_WAVE: usize = 3; //Spawn Large Enemies Every 3 Waves
pub const WAVE_COLOSSAL_ENEMY_WAVE: usize = 5; //Spawn Large Enemies Every 5 Waves
pub const WAVE_BOSS_ENEMY_WAVE: usize = 10; //Spawn Boss Enemy Every 10 Waves

//Enemy Configs
pub struct EnemyConfig {
    pub health: f32,
    pub damage: f32,
    pub sides: usize,
    pub size: f32,
    pub reward: usize,
    pub speed: f32,
    pub movement: MovementType,
}

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}

pub fn rand_type(allowed_types: &[MovementType]) -> MovementType {
    let mut rng = rand::rng();
    *allowed_types
        .choose(&mut rng)
        .unwrap_or(&MovementType::Line)
}

impl EnemyType {
    pub fn get_config(&self) -> EnemyConfig {
        let level: f32 = rand::random_range(0.0..1.0);

        match self {
            EnemyType::Normal => EnemyConfig {
                health:   lerp(10.0, 20.0, level),
                damage:   lerp(2.0, 10.0, level),
                size:     lerp(15.0, 30.0, level),
                speed:    lerp(250.0, 100.0, level), 
                reward:   lerp(10.0, 20.0, level) as usize,
                sides:    3,
                movement: MovementType::Line,
            },
            EnemyType::Unique => EnemyConfig {
                health:   lerp(30.0, 5.0, level),
                damage:   lerp(30.0, 5.0, level),
                size:     lerp(20.0, 5.0, level),
                speed:    lerp(350.0, 50.0, level), 
                reward:   lerp(5.0, 15.0, level) as usize,
                sides:    4,
                movement: MovementType::Switch,
            },
            EnemyType::Large => EnemyConfig {
                health:   lerp(25.0, 100.0, level),
                damage:   lerp(10.0, 50.0, level),
                size:     lerp(20.0, 50.0, level),
                speed:    lerp(125.0, 50.0, level),
                reward:   lerp(40.0, 80.0, level) as usize,
                sides:    6,
                movement: rand_type(&[MovementType::Line, MovementType::Circular]),
            },
            EnemyType::Colossal => EnemyConfig {
                health:   lerp(250.0, 500.0, level),
                damage:   lerp(50.0, 100.0, level),
                size:     lerp(80.0, 125.0, level),
                speed:    lerp(24.0, 12.0, level),
                reward:   lerp(15.0, 40.0, level) as usize,
                sides:    8,
                movement: rand_type(&[MovementType::Line, MovementType::ZigZag]),
            },
            EnemyType::Boss => EnemyConfig {
                health:   lerp(400.0, 1000.0, level),
                damage:   lerp(400.0, 800.0, level),
                size:     lerp(150.0, 350.0, level),
                speed:    lerp(50.0, 10.0, level), 
                reward:   250,
                sides:    100,
                movement: rand_type(&[MovementType::Line, MovementType::ZigZag, MovementType::Switch]),
            },
        }
    }
}

//helper func to turn 250.0 to 250 and 25.25 to 25.25 - keeps UI clean
//rounds to 2 decimals max
pub fn format(val: f32) -> String {
    format!("{:.2}", val).trim_end_matches('0').trim_end_matches('.').to_string()
}