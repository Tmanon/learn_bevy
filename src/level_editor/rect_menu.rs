use crate::actions::MainAction;
use crate::body_bundles::*;
use crate::level_editor::menu_bundles::*;
use crate::states::AppState;

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn menu_system(
    mut next_state: ResMut<NextState<AppState>>,
    query_action: Query<&ActionState<MainAction>>,
) {
    let action_state = &query_action.single();
    if action_state.just_pressed(MainAction::BuildMenu) {
        println!("presed b in menu");
        next_state.set(AppState::InGame);
    }
}

pub fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &ButtonList,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (button, interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = "You Can Realease".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                match button {
                    ButtonList::SpawnBody => {
                        commands.spawn((BodyList::Body, BodyBundle::default()));
                    }
                    ButtonList::SpawnPlayer => {
                        commands.spawn((BodyList::Player, PlayerBundle::default()));
                    }
                    ButtonList::SpawnWall => {
                        commands.spawn((BodyList::Wall, WallBundle::default()));
                    }
                    ButtonList::Play => {
                        next_state.set(AppState::InGame);
                    }
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
                match button {
                    ButtonList::SpawnBody => text.sections[0].value = "Spawn Ball".to_string(),
                    ButtonList::SpawnPlayer => text.sections[0].value = "Spawn Player".to_string(),
                    ButtonList::SpawnWall => text.sections[0].value = "Spawn Wall".to_string(),
                    ButtonList::Play => text.sections[0].value = "Play".to_string(),
                };
            }
        };
    }
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

#[derive(Component)]
pub enum BodyList {
    Body,
    Player,
    Wall,
}

#[derive(Component)]
pub enum ButtonList {
    SpawnBody,
    SpawnPlayer,
    SpawnWall,
    Play,
}

pub fn rect_menu_setup(mut commands: Commands) {
    commands.spawn(MyNode::default().0).with_children(|parent| {
        parent
            .spawn((ButtonList::SpawnBody, MyButton::default().0))
            .with_children(|parent| {
                parent.spawn(MyText::default().0);
            });
        parent
            .spawn((ButtonList::SpawnPlayer, MyButton::default().0))
            .with_children(|parent| {
                parent.spawn(MyText::default().0);
            });
        parent
            .spawn((ButtonList::SpawnWall, MyButton::default().0))
            .with_children(|parent| {
                parent.spawn(MyText::default().0);
            });
        parent
            .spawn((ButtonList::Play, MyButton::default().0))
            .with_children(|parent| {
                parent.spawn(MyText::default().0);
            });
    });
}

pub fn rect_menu_despawn(mut commands: Commands, menu_query: Query<Entity, With<Node>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}
