use bevy::prelude::*;
use crate::player::Player;
use crate::config;

#[derive(Component)]
pub struct HealthBarText;

#[derive(Component)]
pub struct HealthValueText;

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
            row_gap: Val::Px(2.0),
            ..default()
        })
        .with_children(|inner| {
            // HP Text On Top of UI
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

            //HP Bars
            inner.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                HealthBarText,
            ))
            .with_children(|bar_root| {
                bar_root.spawn((
                    Text::new(""),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                bar_root.spawn((
                    Text::new(""),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
                ));
            });
        });
    });
}

fn update_player_ui(
    player_query: Query<&Player>,
    bar_parent_query: Query<&Children, With<HealthBarText>>,
    // We use Without to ensure queries are disjoint and don't conflict
    mut val_query: Query<&mut Text, (With<HealthValueText>, Without<HealthBarText>)>,
    mut text_query: Query<&mut Text, (Without<HealthValueText>, Without<HealthBarText>)>,
) {
    //If player is Invalid; Exit
    let Ok(player) = player_query.single() else { return; };

    //Update HP Text
    if let Ok(mut text) = val_query.single_mut() {
        text.0 = format!("{:.0} HP", player.health);
    }

    //Update HP Bar
    if let Ok(children) = bar_parent_query.single() {
        let sections = (player.max_health / config::UI_HEALTH_DIV).floor() as usize;
        let current_health_blocks = (player.health / config::UI_HEALTH_DIV).floor() as usize;
        let current_health_blocks = current_health_blocks.min(sections);
        let remaining = sections.saturating_sub(current_health_blocks);

        println!("{} -> {} - {}", sections, current_health_blocks, remaining);

        // Update filled portion (Child 0)
        if let Some(&child) = children.get(0) {
            if let Ok(mut text) = text_query.get_mut(child) {
                text.0 = "█ ".repeat(current_health_blocks);
            }
        }
        // Update empty portion (Child 1)
        if let Some(&child) = children.get(1) {
            if let Ok(mut text) = text_query.get_mut(child) {
                text.0 = "█ ".repeat(remaining);
            }
        }
    }
}