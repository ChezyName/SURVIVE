use bevy::prelude::*;
use crate::enemies::{EnemyType, MovementType};
use rand;

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

//Wave Spawning
pub const WAVE_ENEMIES_PER_FRAME: [i32; 2] = [5, 50]; //How Many Enemies (Min - Max) Can Spawn Per Frame
pub const WAVE_ENEMIES_TIME_PER_FRAME: [i32; 2] = [50, 1000]; //How Much Time Between The Frames (Min - Max) in ms
pub const WAVE_ENEMIES_PER_WAVE: usize = 3; //How Many Enemies Per Wave
pub const WAVE_LARGE_ENEMY_WAVE: usize = 5; //Spawn Large Enemies Every 5 Waves x Wave 
pub const WAVE_BOSS_ENEMY_WAVE: usize = 10; //Spawn Boss Enemy Every 10 Waves x Wave

//Enemy Configs
pub struct EnemyConfig {
    pub health: f32,
    pub damage: f32,
    pub sides: usize,
    pub size: f32,
    pub is_boss: bool,
    pub reward: usize,
    pub speed: f32,
    pub movement: MovementType,
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
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
                is_boss:  false,
                movement: MovementType::Line,
            },
            EnemyType::Large => EnemyConfig {
                health:   lerp(25.0, 100.0, level),
                damage:   lerp(10.0, 50.0, level),
                size:     lerp(20.0, 50.0, level),
                speed:    lerp(125.0, 50.0, level),
                reward:   lerp(50.0, 100.0, level) as usize,
                sides:    4,
                is_boss:  false,
                movement: MovementType::Line,
            },
            EnemyType::Boss => EnemyConfig {
                health:   lerp(250.0, 500.0, level),
                damage:   lerp(50.0, 100.0, level),
                size:     lerp(80.0, 125.0, level),
                speed:    lerp(24.0, 12.0, level),
                reward:   lerp(375.0, 750.0, level) as usize,
                sides:    30,
                is_boss:  true,
                movement: MovementType::Line,
            },
        }
    }
}