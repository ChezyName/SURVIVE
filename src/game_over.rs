use bevy::prelude::*;
use crate::enemy::Enemy;
use crate::projectile::{ExplosionVfx, Projectile};
use crate::wave::Wave;
use crate::player::Player;
use crate::{AppState, GameState};
use crate::menu_base::{spawn_menu, despawn_menu, any_key_continue};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over)
           .add_systems(Update, any_key_continue(AppState::InGame).run_if(in_state(AppState::GameOver)))
           .add_systems(OnExit(AppState::GameOver), despawn_game_over);
    }
}

pub fn calculate_score(game_state: &GameState) -> f64 {
    let round_score = (game_state.round as f64).powf(1.8) * 100.0;
    let kill_score = game_state.enemies_killed.iter().map(|(enemy_type, count)| {
        let config = enemy_type.get_config();
        let base = config.score_multi as f64 * 50.0;
        base * (*count as f64)
    }).sum::<f64>();
    let time_bonus = (game_state.playtime_secs as f64).sqrt() * 10.0;
    let money_bonus = game_state.money as f64 * 0.5;
    let total = round_score + kill_score + time_bonus + money_bonus;
    (total * 100.0).round() / 100.0
}

fn spawn_game_over(mut commands: Commands, asset_server: Res<AssetServer>, game_state: Res<GameState>) {
    let font = asset_server.load("fonts/FiraCode-SemiBold.ttf");
    let score_str = format!("Score: {:.2}", calculate_score(&game_state));
    let wave_str  = format!("Wave: {}", game_state.round);
    let time_str  = format!("Time Alive: {}", crate::gamestate::fmt_playtime(game_state.playtime_secs));

    spawn_menu(
        &mut commands,
        font,
        "GAME OVER",
        Color::srgba(1.0, 0.2, 0.2, 1.0),
        &[
            (&score_str, 36.0),
            (&wave_str,  24.0),
            (&time_str,  24.0),
        ],
        "Press any key to restart",
    );
}

fn despawn_game_over(
    mut commands: Commands,
    menu_query: Query<Entity, With<crate::menu_base::MenuScreen>>,
    enemy_query: Query<Entity, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    wave_query: Query<Entity, With<Wave>>,
    explosion_query: Query<Entity, With<ExplosionVfx>>,
    mut player_query: Query<&mut Player>,
    mut game_state: ResMut<GameState>,
) {
    for entity in &menu_query { commands.entity(entity).despawn(); }
    for entity in &enemy_query { commands.entity(entity).despawn(); }
    for entity in &projectile_query { commands.entity(entity).despawn(); }
    for entity in &wave_query { commands.entity(entity).despawn(); }
    for entity in &explosion_query { commands.entity(entity).despawn(); }

    if let Ok(mut player) = player_query.single_mut() {
        *player = Player::default();
    }

    *game_state = GameState::default();
}