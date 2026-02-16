mod player;
mod projectile;
mod config;
mod enemy;
mod gamestate;
mod ui;
mod items;
mod wave_manager;
mod enemies;

use bevy::prelude::*;
use crate::gamestate::GameState;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GameState::default())
        .add_plugins(DefaultPlugins)
        .init_resource::<wave_manager::WaveStatus>()
        .add_systems(Startup, wave_manager::start_wave)
        .add_systems(Update, (wave_manager::check_wave_end, wave_manager::spawn_tick_system))
        .add_systems(Startup, |mut cmd: Commands| { cmd.spawn(Camera2d); })//Camera
        .add_plugins(player::PlayerPlugin)// Player
        .add_plugins(ui::player::PlayerUIPlugin) //Player UI
        .add_plugins(projectile::ProjectilePlugin)// Projectiles
        .add_plugins(enemy::EnemyPlugin)// Enemies
        .run();
}