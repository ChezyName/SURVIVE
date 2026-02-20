use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::items::{Item, ItemFactory};
use crate::player::Player;
use crate::{AppState, GameState};

#[derive(Component)]
pub struct ShopMenu;

#[derive(Component)]
pub struct PurchaseButton(pub Box<dyn Item>);

#[derive(Component)]
pub struct SkipButton;

#[derive(Component)]
pub struct MoneyText;

#[derive(Component)]
pub struct CardTitleText(pub String);

#[derive(Component)]
pub struct DisabledButton;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Shop), spawn_shop)
            .add_systems(
                Update,
                (shop_interaction, skip_interaction).run_if(in_state(AppState::Shop)),
            )
            .add_systems(OnExit(AppState::Shop), despawn_shop);
    }
}

pub fn spawn_shop(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
    mut player_query: Query<&mut Player>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraCode-SemiBold.ttf");
    let mut rng = rand::rng();
    let mut player = player_query.single_mut().ok();

    let available: Vec<fn() -> Box<dyn Item>> = inventory::iter::<ItemFactory>()
        .map(|f| f.0)
        .filter(|factory| {
            let item = factory();

            let unique_check = if item.is_unique() {
                !game_state.has_item(item.name().as_str())
            } else {
                true
            };

            let can_buy = if let Some(ref mut p) = player {
                item.can_buy(&mut *p)
            } else {
                false
            };

            unique_check && can_buy
        })
        .collect();

    let chosen: Vec<Box<dyn Item>> = available
        .into_iter()
        .sample(&mut rng, 3)
        .into_iter()
        .map(|factory| factory())
        .collect();

    let has_items = !chosen.is_empty();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(40.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
            ShopMenu,
        ))
        .with_children(|root: &mut RelatedSpawnerCommands<ChildOf>| {
            root.spawn((
                Text::new("SHOP"),
                TextFont { font: font.clone(), font_size: 48.0, ..default() },
                TextColor(Color::WHITE),
            ));
            root.spawn((
                Text::new(format!("${}", game_state.money)),
                TextFont { font: font.clone(), font_size: 28.0, ..default() },
                TextColor(Color::srgba(1.0, 0.84, 0.0, 1.0)),
                MoneyText,
            ));

            root.spawn(Node {
                width: Val::Px(790.0),
                height: Val::Px(350.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.0),
                ..default()
            })
            .with_children(|area: &mut RelatedSpawnerCommands<ChildOf>| {
                if has_items {
                    for item in chosen {
                        let count = game_state.item_count(item.name().as_str());
                        if let Some(ref mut p) = player {
                            spawn_item_card(area, &font, item, count, &mut *p, &game_state);
                        }
                    }
                } else {
                    area.spawn((
                        Text::new("No new items available.\nYou already have everything!"),
                        TextFont { font: font.clone(), font_size: 22.0, ..default() },
                        TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                }
            });

            root.spawn((
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(40.0), Val::Px(14.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
                SkipButton,
            ))
            .with_children(|btn: &mut RelatedSpawnerCommands<ChildOf>| {
                btn.spawn((
                    Text::new("Leave Shop"),
                    TextFont { font: font.clone(), font_size: 20.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

fn spawn_item_card(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: &Handle<Font>,
    item: Box<dyn Item>,
    owned_count: usize,
    player: &mut Player,
    game_state: &GameState,
) {
    let item_name = item.name();
    let is_unique = item.is_unique();

    let already_owned_unique = is_unique && game_state.has_item(item_name.as_str());
    let can_afford = item.can_buy(player);
    let disabled = already_owned_unique || !can_afford;

    let card_color = if disabled {
        Color::srgba(0.08, 0.08, 0.08, 1.0)
    } else {
        Color::srgba(0.15, 0.15, 0.15, 1.0)
    };

    let title = if !is_unique && owned_count > 0 {
        format!("{} x{}", item_name, owned_count)
    } else {
        item_name.clone()
    };

    let mut card = parent.spawn((
        Button,
        Node {
            width: Val::Px(250.0),
            height: Val::Px(350.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(card_color),
        PurchaseButton(item.clone_box()),
    ));

    if disabled {
        card.insert(DisabledButton);
    }

    card.with_children(|card: &mut RelatedSpawnerCommands<ChildOf>| {
        let text_color = if disabled {
            TextColor(Color::srgba(0.4, 0.4, 0.4, 1.0))
        } else {
            TextColor(Color::WHITE)
        };

        if is_unique {
            card.spawn((
                Text::new(title),
                TextFont { font: font.clone(), font_size: 24.0, ..default() },
                text_color,
            ));
        } else {
            card.spawn((
                Text::new(title),
                TextFont { font: font.clone(), font_size: 24.0, ..default() },
                text_color,
                CardTitleText(item_name.clone()),
            ));
        }

        card.spawn((
            Text::new(item.description()),
            TextFont { font: font.clone(), font_size: 14.0, ..default() },
            TextColor(Color::srgba(0.7, 0.7, 0.7, 1.0)),
        ));

        let cost_color = if disabled {
            Color::srgba(0.5, 0.42, 0.0, 1.0)
        } else {
            Color::srgba(1.0, 0.84, 0.0, 1.0)
        };

        card.spawn((
            Text::new(format!("${}", item.cost())),
            TextFont { font: font.clone(), font_size: 22.0, ..default() },
            TextColor(cost_color),
        ));
    });
}

pub fn shop_interaction(
    mut interaction_query: Query<
        (&Interaction, &PurchaseButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<&mut Player>,
    mut money_query: Query<&mut Text, With<MoneyText>>,
    mut title_query: Query<(&CardTitleText, &mut Text), Without<MoneyText>>,
) {
    for (interaction, button_data, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let item = &button_data.0;
                if game_state.money >= item.cost() {
                    if let Ok(mut player) = player_query.single_mut() {
                        game_state.money -= item.cost();
                        item.apply(&mut player);
                        game_state.record_purchase(item.name());

                        let item_name = item.name();
                        let new_count = game_state.item_count(item_name.as_str());

                        // Update money display
                        if let Ok(mut text) = money_query.single_mut() {
                            **text = format!("${}", game_state.money);
                        }

                        // Update the title on this item's card
                        for (title, mut text) in &mut title_query {
                            if title.0 == item_name {
                                **text = format!("{} x{}", item_name, new_count);
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => bg_color.0 = Color::srgba(0.28, 0.28, 0.28, 1.0),
            Interaction::None    => bg_color.0 = Color::srgba(0.15, 0.15, 0.15, 1.0),
        }
    }
}

pub fn skip_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SkipButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => next_state.set(AppState::InGame),
            Interaction::Hovered => bg_color.0 = Color::srgba(0.35, 0.35, 0.35, 1.0),
            Interaction::None    => bg_color.0 = Color::srgba(0.2,  0.2,  0.2,  1.0),
        }
    }
}

pub fn despawn_shop(mut commands: Commands, query: Query<Entity, With<ShopMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}