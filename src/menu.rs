#![allow(clippy::type_complexity)]

use crate::GameState;
use bevy::{app::AppExit, prelude::*};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct MenuNode;

#[derive(Component)]
enum ButtonType {
    Play,
    Quit,
}

fn button_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ButtonType,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                match button_type {
                    ButtonType::Play => next_state.set(GameState::InGame),
                    ButtonType::Quit => exit.send(AppExit),
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup_menu(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(150.0),
        height: Val::Px(65.0),
        border: UiRect::all(Val::Px(2.0)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
    };
    let text_style = TextStyle {
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
        ..default()
    };

    let panel_style = Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        ..default()
    };
    commands
        .spawn(NodeBundle {
            style: panel_style,
            ..default()
        })
        .insert(MenuNode)
        .with_children(|parent| {
            //Start button
            parent
                .spawn(ButtonBundle {
                    style: button_style.clone(),
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(ButtonType::Play)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Play", text_style.clone()));
                });

            //Quit button
            parent
                .spawn(ButtonBundle {
                    style: button_style.clone(),
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(ButtonType::Quit)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Quit", text_style.clone()));
                });
        });
}

fn cleanup_menu(mut commands: Commands, menu_panel: Query<Entity, With<MenuNode>>) {
    for ent in menu_panel.iter() {
        commands.entity(ent).despawn_recursive()
    }
}
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, button_system.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}
