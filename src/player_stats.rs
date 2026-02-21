use bevy::prelude::*;
use crate::player::Player;
use crate::{AppState, GameState, gamestate};
use crate::config::format;
use crate::enemy::EnemyType;

#[derive(Component)]
pub struct StatsPanel;

pub struct PlayerStatsPlugin;

impl Plugin for PlayerStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_stats_panel.run_if(in_state(AppState::InGame).or(in_state(AppState::GameOver).or(in_state(AppState::Shop)))),
        );
    }
}

fn toggle_stats_panel(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    panel_query: Query<Entity, With<StatsPanel>>,
    player_query: Query<&Player>,
    game_state: Res<GameState>,
    app_state: Res<State<AppState>>,
    mut time: ResMut<Time<Virtual>>,
) {
    if *app_state.get() == AppState::MainMenu {
        return;
    }

    if keys.just_pressed(KeyCode::Tab) {
        let font: Handle<Font> = asset_server.load("fonts/FiraCode-SemiBold.ttf");
        let Ok(player) = player_query.single() else { return };
        time.pause();
        spawn_stats_panel(&mut commands, &font, player, &game_state);
    }

    if keys.just_released(KeyCode::Tab) {
        time.unpause();
        for entity in &panel_query {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_stats_panel(
    commands: &mut Commands,
    font: &Handle<Font>,
    player: &Player,
    game_state: &GameState,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(320.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            StatsPanel,
            GlobalZIndex(100),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("GAME PAUSED"),
                TextFont { font: font.clone(), font_size: 22.0, ..default() },
                TextColor(Color::srgba(1.0, 1.0, 0.0, 1.0)),
            ));

            spawn_divider(root);

            spawn_stat_row(root, font, "Time Alive", &gamestate::fmt_playtime(game_state.playtime_secs));

            let order = [EnemyType::Normal, EnemyType::Unique, EnemyType::Large, EnemyType::Colossal, EnemyType::Boss];
            for enemy_type in &order {
                if let Some(count) = game_state.enemies_killed.get(enemy_type) {
                    spawn_stat_row(root, font, &format!("{} Enemies Killed", enemy_type.name()), &format(*count as f32));
                }
            }

            spawn_stat_row(root, font, "Total Enemies Killed", &format(game_state.total_enemies_killed as f32));

            spawn_divider(root);

            let stats = [
                ("Health",       format!("{} / {}", format(player.health), format(player.max_health))),
                ("Damage",       format(player.damage)),
                ("Fire Rate",    format!("{}RPM", format(player.fire_rate))),
                ("Bullet Speed", format!("{}m/s", format(player.bullet_speed))),
                ("Pellets",      format!("{}", player.pellets)),
                ("Spread",       format!("{}°", format(player.bullet_spread))),
                ("Life Steal",   format!("{}%", format(player.life_steal))),
            ];

            for (label, value) in &stats {
                spawn_stat_row(root, font, label, value);
            }

            spawn_divider(root);

            if game_state.item_counts.is_empty() {
                root.spawn((
                    Text::new("No items purchased yet."),
                    TextFont { font: font.clone(), font_size: 13.0, ..default() },
                    TextColor(Color::srgba(0.5, 0.5, 0.5, 1.0)),
                ));
            } else {
                let mut sorted: Vec<(&String, &usize)> = game_state.item_counts.iter().collect();
                sorted.sort_by_key(|(name, _)| name.as_str());

                for (name, count) in sorted {
                    let count_str = if *count >= 1 { format!("{}x", count) } else { String::new() };
                    spawn_item_row(root, font, name, &count_str);
                }
            }
        });
}

fn spawn_divider(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<bevy::ecs::hierarchy::ChildOf>,
) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            margin: UiRect::vertical(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.15)),
    ));
}

fn spawn_stat_row(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<bevy::ecs::hierarchy::ChildOf>,
    font: &Handle<Font>,
    label: &str,
    value: &str,
) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont { font: font.clone(), font_size: 14.0, ..default() },
                TextColor(Color::srgba(0.75, 0.75, 0.75, 1.0)),
            ));
            row.spawn((
                Text::new(value),
                TextFont { font: font.clone(), font_size: 14.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_item_row(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<bevy::ecs::hierarchy::ChildOf>,
    font: &Handle<Font>,
    name: &str,
    count: &str,
) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(name),
                TextFont { font: font.clone(), font_size: 14.0, ..default() },
                TextColor(Color::srgba(0.85, 0.85, 0.85, 1.0)),
            ));
            row.spawn((
                Text::new(count),
                TextFont { font: font.clone(), font_size: 14.0, ..default() },
                TextColor(Color::srgba(1.0, 1.0, 0.0, 1.0)),
            ));
        });
}