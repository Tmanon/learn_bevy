use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::actions::*;
use crate::body_bundles::*;
use crate::constants::*;

pub fn spawn_players(mut movement_properties: ResMut<MovementProperties>, mut commands: Commands) {
    movement_properties.boost = BOOST;
    movement_properties.jump = JUMP;
    movement_properties.torque = TORQUE;
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
