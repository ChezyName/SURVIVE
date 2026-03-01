//basic menu for re-use
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;

#[derive(Component)]
pub struct MenuScreen;

pub fn spawn_menu(
    commands: &mut Commands,
    font: Handle<Font>,
    title: &str,
    title_color: Color,
    lines: &[(&str, f32)],
    prompt: &str,
) {
    commands
        .spawn((
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
            MenuScreen,
        ))
        .with_children(|root| {
            root.spawn((
                Text::new(title),
                TextFont { font: font.clone(), font_size: 72.0, ..default() },
                TextColor(title_color),
            ));

            for (text, size) in lines {
                root.spawn((
                    Text::new(*text),
                    TextFont { font: font.clone(), font_size: *size, ..default() },
                    TextColor(Color::WHITE),
                ));
            }

            root.spawn((
                Text::new(prompt),
                TextFont { font: font.clone(), font_size: 18.0, ..default() },
                TextColor(Color::linear_rgba(1.0, 1.0, 1.0, 0.5)),
            ));
        });
}

pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn any_key_continue<S: States + FreelyMutableState>(next: S) -> impl Fn(
    Res<ButtonInput<KeyCode>>,
    Res<ButtonInput<MouseButton>>,
    ResMut<NextState<S>>,
) {
    move |keys, mouse, mut next_state| {
        let pressed = keys.get_just_pressed().next().is_some()
            || mouse.get_just_pressed().next().is_some();
        if pressed {
            *next_state = NextState::Pending(next.clone());
        }
    }
}