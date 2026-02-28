use bevy::prelude::*;
use crate::enemy::{Enemy, EnemyType};
use crate::projectile::ExplosionVfx;
use crate::wave::Wave;
use crate::player::Player;
use crate::projectile::Projectile;
use crate::{AppState, GameState};

#[derive(Component)]
pub struct GameOverMenu;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over).add_systems(Update, any_key_restart.run_if(in_state(AppState::GameOver))).add_systems(OnExit(AppState::GameOver), despawn_game_over);
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

fn spawn_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraCode-SemiBold.ttf");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
        GameOverMenu,
    ))
    .with_children(|root| {
        root.spawn((
            Text::new("GAME OVER"),
            TextFont { font: font.clone(), font_size: 72.0, ..default() },
            TextColor(Color::srgba(1.0, 0.2, 0.2, 1.0)),
        ));

        root.spawn((
            Text::new(format!("Score: {:.2}", calculate_score(&game_state))),
            TextFont { font: font.clone(), font_size: 36.0, ..default() },
            TextColor(Color::srgba(1.0, 0.84, 0.0, 1.0)),
        ));

        root.spawn((
            Text::new(format!("Wave: {}", game_state.round)),
            TextFont { font: font.clone(), font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
        ));

        root.spawn((
            Text::new(format!("Time Alive: {}", crate::gamestate::fmt_playtime(game_state.playtime_secs))),
            TextFont { font: font.clone(), font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
        ));

        root.spawn((
            Text::new("Press any key to restart"),
            TextFont { font: font.clone(), font_size: 18.0, ..default() },
            TextColor(Color::linear_rgba(1.0, 1.0, 1.0, 0.5)),
        ));
    });
}

fn any_key_restart(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let pressed = keys.get_just_pressed().next().is_some() || mouse.get_just_pressed().next().is_some();
    if pressed { next_state.set(AppState::InGame); }
}

//destroy all
fn despawn_game_over(
    mut commands: Commands,
    game_over_query: Query<Entity, With<GameOverMenu>>,
    enemy_query: Query<Entity, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    wave_query: Query<Entity, With<Wave>>,
    explosion_query: Query<Entity, With<ExplosionVfx>>,
    mut player_query: Query<&mut Player>,
    mut game_state: ResMut<GameState>,
) {
    for entity in &game_over_query { commands.entity(entity).despawn(); }
    for entity in &enemy_query { commands.entity(entity).despawn(); }
    for entity in &projectile_query { commands.entity(entity).despawn(); }
    for entity in &wave_query { commands.entity(entity).despawn(); }
    for entity in &explosion_query { commands.entity(entity).despawn(); }

    if let Ok(mut player) = player_query.single_mut() {
        *player = Player::default();
    }

    *game_state = GameState::default();
}