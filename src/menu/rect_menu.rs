use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::actions::MainAction;
use crate::body_bundles::*;
use crate::menu::menu_bundles::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &BodyList,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (body_button, interaction, mut color, mut border_color, children) in &mut interaction_query
    {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "You Can Realease".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                match body_button {
                    BodyList::Body => commands.spawn(BodyBundle::default()),
                    BodyList::Player => commands.spawn(PlayerBundle::default()),
                    BodyList::Wall => commands.spawn((BodyList::Wall, WallBundle::default())),
                };
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
pub enum BodyList {
    Body,
    Player,
    Wall,
}

//pub enum Bodies {
//    Body(BodyBundle),
//    Player(PlayerBundle),
//    Wall(WallBundle),
//}
//
//impl Bodies {
//    fn vector() -> Vec<Bodies> {
//        let bodies_enum: Vec<Bodies> = vec![
//            Bodies::Body(BodyBundle::default()),
//            Bodies::Player(PlayerBundle::default()),
//            Bodies::Wall(WallBundle::default()),
//        ];
//        bodies_enum
//    }
//}

pub fn rect_menu_setup(
    mut commands: Commands,
    query_action: Query<&ActionState<MainAction>>,
    query_menu: Query<Entity, With<MyNode>>,
) {
    for action_state in &query_action {
        if action_state.just_pressed(MainAction::BuildMenu) {
            if !query_menu.is_empty() {
                commands.entity(query_menu.single()).despawn_recursive();
            } else {
                commands.spawn(MyNode::default().0).with_children(|parent| {
                    parent
                        .spawn((BodyList::Body, MyButton::default().0))
                        .with_children(|parent| {
                            parent.spawn(MyText::default().0);
                        });
                    parent
                        .spawn((BodyList::Player, MyButton::default().0))
                        .with_children(|parent| {
                            parent.spawn(MyText::default().0);
                        });
                    parent
                        .spawn((BodyList::Wall, MyButton::default().0))
                        .with_children(|parent| {
                            parent.spawn(MyText::default().0);
                        });
                });
            }
        }
    }
}
