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
use bevy::ecs::schedule::ApplyDeferred;
use crate::gamestate::GameState;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    MainMenu,
    #[default] //starts in game for now
    InGame,
    Shop,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GameState::default())
        .add_plugins(DefaultPlugins)
        .init_resource::<wave_manager::WaveStatus>()
        .add_systems(Startup, wave_manager::start_wave)
        .add_systems(Update, (wave_manager::spawn_tick_system, ApplyDeferred, wave_manager::check_wave_end).chain())
        .add_systems(Startup, |mut cmd: Commands| { cmd.spawn(Camera2d); })//Camera
        .add_plugins(player::PlayerPlugin)// Player
        .add_plugins(ui::player::PlayerUIPlugin) //Player UI
        .add_plugins(projectile::ProjectilePlugin)// Projectiles
        .add_plugins(enemy::EnemyPlugin)// Enemies
        .run();
}