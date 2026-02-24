use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::hover::HoverMap;
use crate::player::Player;
use crate::{AppState, GameState, gamestate};
use crate::config::format;
use crate::enemy::EnemyType;

#[derive(Component)]
pub struct StatsPanel;

#[derive(Component)]
pub struct ScrollArea;

#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
struct Scroll {
    entity: Entity,
    delta: Vec2,
}

pub struct PlayerStatsPlugin;

impl Plugin for PlayerStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_stats_panel.run_if(in_state(AppState::InGame).or(in_state(AppState::GameOver).or(in_state(AppState::Shop)))),
        ).add_systems(Update, send_scroll_events).add_observer(on_scroll_handler);
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
    //Stat only item list
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
                border: UiRect::right(Val::Px(1.0)),
                ..default()
            },
            BorderColor {
                top: Color::NONE,
                right: Color::srgba(1.0, 1.0, 1.0, 0.2),
                bottom: Color::NONE,
                left: Color::NONE,
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

            // Scrollable container
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    overflow: Overflow::scroll_y(),
                    row_gap: Val::Px(6.0),
                    ..default()
                },
                ScrollArea,
            ))
            .with_children(|scroll| {
                spawn_stat_row(scroll, font, "Time Alive", &gamestate::fmt_playtime(game_state.playtime_secs));

                let order = [EnemyType::Normal, EnemyType::Unique, EnemyType::Large, EnemyType::Colossal, EnemyType::Boss];
                for enemy_type in &order {
                    if let Some(count) = game_state.enemies_killed.get(enemy_type) {
                        spawn_stat_row(scroll, font, &format!("{} Enemies Killed", enemy_type.name()), &format(*count as f32));
                    }
                }

                spawn_stat_row(scroll, font, "Total Enemies Killed", &format(game_state.total_enemies_killed as f32));

                spawn_divider(scroll);

                let stats = [
                    ("Health",                  format!("{} / {}", format(player.health), format(player.max_health))),
                    ("Damage",                  format(player.damage)),
                    ("Fire Rate",               format!("{}RPM", format(player.fire_rate))),
                    ("Bullet Speed",            format!("{}m/s", format(player.bullet_speed))),
                    ("Pellets",                 format!("{}", player.pellets)),
                    ("Spread",                  format!("{}°", format(player.bullet_spread))),
                    ("Penetration",             format!("{}", format(player.bullet_pierce as f32))),
                    ("Bullet Size",             format!("{}%", format(player.bullet_size))),
                    ("Life Steal",              format!("{}%", format(player.life_steal))),
                    ("Critical Strike Chance",  format!("{}%", format(player.crit_percent))),
                ];

                for (label, value) in &stats {
                    spawn_stat_row(scroll, font, label, value);
                }
            });
        });

    //item only
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(320.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(6.0),
                border: UiRect::left(Val::Px(1.0)),
                ..default()
            },
            BorderColor {
                top: Color::NONE,
                left: Color::srgba(1.0, 1.0, 1.0, 0.2),
                bottom: Color::NONE,
                right: Color::NONE,
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            StatsPanel,
            GlobalZIndex(100),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("ITEMS"),
                TextFont { font: font.clone(), font_size: 22.0, ..default() },
                TextColor(Color::srgba(1.0, 1.0, 0.0, 1.0)),
            ));

            spawn_divider(root);

            // Scrollable container
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    overflow: Overflow::scroll_y(),
                    row_gap: Val::Px(6.0),
                    ..default()
                },
                ScrollArea,
            ))
            .with_children(|scroll| {
                if game_state.item_counts.is_empty() {
                    scroll.spawn((
                        Text::new("No items purchased yet."),
                        TextFont { font: font.clone(), font_size: 13.0, ..default() },
                        TextColor(Color::srgba(0.5, 0.5, 0.5, 1.0)),
                    ));
                } else {
                    let mut sorted: Vec<(&String, &usize)> = game_state.item_counts.iter().collect();
                    sorted.sort_by_key(|(name, _)| name.as_str());

                    for (name, count) in sorted {
                        let count_str = if *count >= 1 { format!("{}x", count) } else { String::new() };
                        spawn_item_row(scroll, font, name, &count_str);
                    }
                }
            });
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

pub fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut commands: Commands,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);

        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= 21.0;
        }

        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                commands.trigger(Scroll { entity, delta });
            }
        }
    }
}

fn on_scroll_handler(
    mut scroll: On<Scroll>,
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode), With<ScrollArea>>,
) {
    let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity) else {
        return;
    };

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();
    let delta = &mut scroll.delta;

    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
        let max = if delta.y > 0. {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.
        };

        if !max {
            scroll_position.y += delta.y;
            delta.y = 0.;
        }
    }

    if *delta == Vec2::ZERO {
        scroll.propagate(false);
    }
}