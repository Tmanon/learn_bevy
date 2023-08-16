use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::actions::*;
use crate::body_bundles::*;

pub fn spawn_players(mut commands: Commands) {
    commands.spawn((
        PlayerBundle {
            body_bundle: BodyBundle::new(Vec2::new(48.0, 60.0), Vec2::ZERO),
            ..default()
        },
        InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::Up, Action::Boost),
                (KeyCode::ControlRight, Action::Jump),
                (KeyCode::Left, Action::RotateLeft),
                (KeyCode::Right, Action::RotateRight),
            ]),
        },
    ));
}
