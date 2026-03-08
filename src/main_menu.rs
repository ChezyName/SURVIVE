use bevy::prelude::*;
use crate::AppState;
use crate::menu_base::{spawn_menu, despawn_menu, any_key_continue};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
           .add_systems(Update, any_key_continue(AppState::InGame).run_if(in_state(AppState::MainMenu)))
           .add_systems(OnExit(AppState::MainMenu), despawn_menu);
    }
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraCode-SemiBold.ttf");
    spawn_menu(
        &mut commands,
        font,
        "PROJECT: SURVIVE",
        Color::srgba(1.0, 1.0, 1.0, 1.0),
        &[(&"LMB Shoot | Mouse to Aim", 16.0)],
        "Press any key to start",
    );
}