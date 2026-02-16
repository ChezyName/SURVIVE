use bevy::prelude::*;
use crate::enemies::{EnemyType, MovementType};

//Player Defaults
pub const PLAYER_MAX_HEALTH: f32 = 100.0;
pub const PLAYER_FIRE_RATE: f32 = 200.0; // 60 / rate where rate = RPM (rounds per min)

//UI Defaults
pub const UI_HEALTH_DIV: f32 = 10.0; //How much health per segment

//Bullet Defaults
pub const BULLET_SPEED: f32 = 250.0;
pub const BULLET_LIFETIME: f32 = 15.0;
pub const BULLET_DAMAGE: f32 = 15.0;
pub const BULLET_HIT_BOX: f32 = 10.0; //For Hitting Enemies

//Basic Enemy Stuff, Multiplies Later
pub const ENEMY_SPAWN_DIST: f32 = 800.0;
pub const ENEMY_HIT_BOX: f32 = 20.0; //For Hitting Player - Slightly Bigger to not get damaged by projectile

//Wave Spawning
pub const WAVE_MAX_ENEMIES_PER_FRAME: usize = 100;
pub const WAVE_ENEMIES_PER_WAVE: usize = 3; //How Many Enemies Per Wave
pub const WAVE_LARGE_ENEMY_WAVE: usize = 5; //Spawn Large Enemies Every 5 Waves x Wave 
pub const WAVE_BOSS_ENEMY_WAVE: usize = 5; //Spawn Boss Enemy Every 10 Waves x Wave

//Enemy Configs
pub struct EnemyConfig {
    pub health: f32,
    pub damage: f32,
    pub sides: usize,
    pub size: f32,
    pub color: Color,
    pub is_boss: bool,
    pub reward: usize,
    pub speed: f32,
    pub movement: MovementType,
}

impl EnemyType {
    pub fn get_config(&self) -> EnemyConfig {
        match self {
            EnemyType::Normal => EnemyConfig {
                health: 10.0,
                damage: 10.0,
                sides: 3,
                size: 15.0,
                color: Color::srgb(1.0,0.0,0.0),
                is_boss: false,
                reward: 10,
                speed: 100.0,
                movement: MovementType::Line,
            },
            EnemyType::Large => EnemyConfig {
                health: 25.0,
                damage: 5.0,
                sides: 4,
                size: 12.0,
                color: Color::srgb(0.2, 0.8, 0.8),
                is_boss: false,
                reward: 15,
                speed: 250.0,
                movement: MovementType::Line,
            },
            EnemyType::Boss => EnemyConfig {
                health: 500.0,
                damage: 250.0,
                sides: 360,
                size: 80.0,
                color: Color::srgb(0.5, 0.0, 1.0),
                is_boss: true,
                reward: 500,
                speed: 50.0,
                movement: MovementType::Line,
            },
        }
    }
}