use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::actions::{MainAction, MovementPropertiesResource};
use crate::game::constants::*;
use crate::states::AppState;

pub fn game_setup(
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

pub fn in_game(
    mut next_state: ResMut<NextState<AppState>>,
    query_action: Query<&ActionState<MainAction>>,
) {
    let action_state = &query_action.single();
    if action_state.just_pressed(MainAction::BuildMenu) {
        println!("presed b");
        next_state.set(AppState::MainMenu);
    }
}
