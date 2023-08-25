use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::actions::{MainAction, MovementPropertiesResource};
use crate::game::constants::*;

pub fn game(
    mut commands: Commands,
    mut movement_properties_resource: ResMut<MovementPropertiesResource>,
) {
    commands.spawn(InputManagerBundle::<MainAction> {
        action_state: ActionState::default(),
        input_map: InputMap::new([(KeyCode::B, MainAction::BuildMenu)]),
    });
    movement_properties_resource.boost = BOOST;
    movement_properties_resource.jump = JUMP;
    movement_properties_resource.torque = TORQUE;
}
