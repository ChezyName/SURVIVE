use bevy::prelude::*;
use crate::player::Player;
use crate::config;
use crate::GameState;

#[derive(Component)]
pub struct HealthValueText;

#[derive(Component)]
pub struct HealthBarText;

#[derive(Component)]
pub struct HealthSegment(pub usize);

#[derive(Component)]
pub struct GameStateText;

pub struct PlayerUIPlugin;

impl Plugin for PlayerUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_ui)
           .add_systems(Update, update_player_ui);
    }
}

fn spawn_player_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraCode-SemiBold.ttf");

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

            // HP Bar Container (The "Bounds")
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
                // Slant to match Overwatch aesthetic
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
    mut text_query: Query<&mut Text, With<HealthValueText>>,
    mut gamestate_query: Query<&mut Text, (With<GameStateText>, Without<HealthValueText>)>,
    game_state: Res<GameState>,
) {
    let Ok(player) = player_query.single() else { return; };

    // Update HP Text
    if let Ok(mut text) = text_query.single_mut() {
        text.0 = format!("{:.0} HP", player.health);
    }

    // Update GS Text
    if let Ok(mut text) = gamestate_query.single_mut() {
        text.0 = format!("[Wave {} | ${}]", game_state.round, format_currency(game_state.money));
    }

    // 1 pip per 10 HP (Change this to adjust granularity)
    let total_capacity = (player.max_health / config::UI_HEALTH_DIV).ceil() as usize; 
    let filled_count = (player.health / config::UI_HEALTH_DIV).ceil() as usize;

    for (segment, mut vis, mut color, mut node) in segment_query.iter_mut() {
        if segment.0 < total_capacity {
            *vis = Visibility::Visible;
            
            // Flex logic: only visible pips get flex_grow
            node.flex_grow = 1.0; 
            node.display = Display::Flex;

            if segment.0 < filled_count {
                color.0 = Color::WHITE; 
            } else {
                color.0 = Color::srgba(1.0, 1.0, 1.0, 0.1); // Empty "ghost" pips
            }
        } else {
            *vis = Visibility::Hidden;
            // Crucial: remove from layout so they don't take up space
            node.flex_grow = 0.0;
            node.display = Display::None; 
        }
    }
}