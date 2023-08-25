use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::actions::MainAction;
use crate::body_bundles::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "You Can Realease".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                commands.spawn(PlayerBundle::default());
            }
            Interaction::Hovered => {
                text.sections[0].value = "If You Want".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Press To Spawn".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

#[derive(Component)]
pub struct MainMenu;

pub fn rect_menu_setup(
    mut commands: Commands,
    query_action: Query<&ActionState<MainAction>>,
    query_menu: Query<Entity, With<MainMenu>>,
) {
    for action_state in &query_action {
        if action_state.just_pressed(MainAction::BuildMenu) {
            if !query_menu.is_empty() {
                commands.entity(query_menu.single()).despawn_recursive();
            } else {
                commands
                    .spawn((
                        MainMenu,
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Start,
                                left: Val::Px(0.),
                                bottom: Val::Px(0.),
                                ..default()
                            },
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(150.0),
                                    height: Val::Px(65.0),
                                    border: UiRect::all(Val::Px(5.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Button",
                                    TextStyle {
                                        font_size: 15.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                        ..default()
                                    },
                                ));
                            });
                    });
            }
        }
    }
}
