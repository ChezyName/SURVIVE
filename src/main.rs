mod player;
mod player_ui;
mod projectile;
mod config;
mod enemy;
mod gamestate;
mod items;
mod wave_manager;
mod shop;
mod player_stats;
mod wave;

use bevy::prelude::*;
use bevy::ecs::schedule::ApplyDeferred;
use crate::gamestate::GameState;
use crate::projectile::HomingSpawnQueue;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default] //starts in game for now
    InGame,
    Shop,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)//Defaults
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GameState::default()).init_state::<AppState>().init_resource::<wave_manager::WaveStatus>().insert_resource(HomingSpawnQueue::default())
        .add_systems(Startup, |mut cmd: Commands| { cmd.spawn(Camera2d); })//Camera
        .add_systems(OnEnter(AppState::InGame), wave_manager::start_wave)
        .add_systems(Update, (wave_manager::spawn_tick_system, ApplyDeferred, wave_manager::check_wave_end, gamestate::update_playtime).chain().run_if(in_state(AppState::InGame)))
        .add_plugins(player::PlayerPlugin).add_plugins(player_ui::PlayerUIPlugin) //Player UI
        .add_plugins(player_stats::PlayerStatsPlugin).add_plugins(projectile::ProjectilePlugin).add_plugins(enemy::EnemyPlugin).add_plugins(wave::WavePlugin)
        //Shop
        .add_plugins(shop::ShopPlugin).run();
}