use bevy::prelude::*;
use crate::player::Player;
use crate::config;
use crate::GameState;
use crate::wave_manager;

#[derive(Component)]
pub struct HealthValueText;

#[derive(Component)]
pub struct HealthBarText;

#[derive(Component)]
pub struct HealthSegment(pub usize);

#[derive(Component)]
pub struct GameStateText;

#[derive(Component)]
pub struct EnemiesText;

pub struct PlayerUIPlugin;

impl Plugin for PlayerUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_ui).add_systems(Update, update_player_ui);
    }
}

fn spawn_player_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("embedded://fonts/FiraCode-SemiBold.ttf");

    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::End,
        padding: UiRect::bottom(Val::Px(40.0)),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(5.0),
            width: Val::Percent(100.0), 
            ..default()
        })
        .with_children(|inner| {
            // HP Text
            inner.spawn((
                Text::new("[100]"),
                TextFont {
                    font_size: 20.0,
                    font: font.clone(),
                    ..default()
                },
                TextColor(Color::WHITE),
                HealthValueText,
            ));

            // HB Bars
            inner.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center, // Centers the pips
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(1.0), // Smaller gap looks better when squished
                    width: Val::Px(500.0),    // Total width available for the bar
                    height: Val::Px(18.0),
                    ..default()
                },
                Transform::from_rotation(Quat::from_rotation_z(-0.12)), 
                HealthBarText,
            ))
            .with_children(|parent| {
                // Spawn a large pool of pips. 
                // We will hide/show them based on max_health.
                for i in 0..200 { 
                    parent.spawn((
                        Node {
                            // This makes them share the parent's width equally
                            flex_grow: 1.0, 
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::WHITE),
                        HealthSegment(i),
                    ));
                }
            });

            inner.spawn((
                Text::new("Wave {} | ${}"),
                TextFont {
                    font_size: 20.0,
                    font: font.clone(),
                    ..default()
                },
                TextColor(Color::WHITE),
                GameStateText,
            ));

            inner.spawn((
                Text::new("- No Enemies Remaining -"),
                TextFont {
                    font_size: 10.0,
                    font: font.clone(),
                    ..default()
                },
                TextColor(Color::WHITE),
                EnemiesText,
            ));
        });
    });
}

pub fn format_currency(value: usize) -> String {
    let val = value as f32;

    if value >= 1_000_000_000_000 {
        format!("{:.1}T", val / 1_000_000_000_000.0)
    } else if value >= 1_000_000_000 {
        format!("{:.1}B", val / 1_000_000_000.0)
    } else if value >= 1_000_000 {
        format!("{:.1}M", val / 1_000_000.0)
    } else if value >= 1_000 {
        format!("{:.1}k", val / 1_000.0)
    } else {
        value.to_string()
    }
    .replace(".0", "")
}

fn update_player_ui(
    player_query: Query<&Player>,
    mut segment_query: Query<(&HealthSegment, &mut Visibility, &mut BackgroundColor, &mut Node)>,
    mut text_query: Query<(&mut Text, &mut TextColor), With<HealthValueText>>,
    mut gamestate_query: Query<&mut Text, (With<GameStateText>, Without<HealthValueText>)>,
    mut enemies_query: Query<&mut Text, (With<EnemiesText>, Without<HealthValueText>, Without<GameStateText>)>,
    game_state: Res<GameState>,
    wave_state: Res<wave_manager::WaveStatus>,
) {
    let Ok(player) = player_query.single() else { return; };
    let ui_color = if player.has_lifeline { Color::srgb(0.937, 0.749, 0.016) } else { Color::WHITE };

    // Update HP Text
    if let Ok((mut text, mut color)) = text_query.single_mut() {
        text.0 = format!("{:.0} HP", player.health);
        color.0 = ui_color;
    }

    // Update GS Text
    if let Ok(mut text) = gamestate_query.single_mut() {
        text.0 = format!("[Wave {} | ${}]", game_state.round, format_currency(game_state.money));
    }

    if let Ok(mut text) = enemies_query.single_mut() {
        let enemy_count = wave_state.enemies_remaining;
        if enemy_count == 1 { text.0 = format!("1 Enemy Remaining"); }
        else if enemy_count <= 0 { text.0 = format!("No Enemy Remaining"); }
        else { text.0 = format!("{} Enemies Remaining", enemy_count); }
    }

    let total_capacity = (player.max_health / config::UI_HEALTH_DIV).ceil() as usize; 
    let filled_count = (player.health / config::UI_HEALTH_DIV).ceil() as usize;

    for (segment, mut vis, mut color, mut node) in segment_query.iter_mut() {
        if segment.0 < total_capacity {
            *vis = Visibility::Visible;
            node.flex_grow = 1.0; 
            node.display = Display::Flex;

            if segment.0 < filled_count {
                color.0 = ui_color; 
            } else {
                color.0 = Color::srgba(1.0, 1.0, 1.0, 0.1);
            }
        } else {
            *vis = Visibility::Hidden;
            node.flex_grow = 0.0;
            node.display = Display::None; 
        }
    }
}