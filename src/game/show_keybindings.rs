use bevy::prelude::*;

//use crate::{actions::PlayerAction, body_bundles::PlayerBundle};
use crate::game::constants::ARENA_SIZE;

pub fn show_keybindings_system(
    mut commands: Commands,
    //query_main_action: Query<&ActionState<MainAction>>,
) {
    //let input_map_player = PlayerBundle::default().input_manager_bundle.input_map;
    let text_style = TextStyle {
        font_size: 60.0,
        color: Color::LIME_GREEN,
        ..default()
    };

    commands.spawn(Text2dBundle {
        text: Text::from_section("Keys: B, Space, UP, LEFT, RIGHT", text_style),
        transform: Transform::from_xyz(ARENA_SIZE.x / 2. - 600., ARENA_SIZE.x / 2. - 500., 1.),
        ..default()
    });
}
