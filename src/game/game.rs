use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

use crate::actions::{MainAction, MovementPropertiesResource};
use crate::game::constants::*;
use crate::states::AppState;

pub fn game_setup(
    mut commands: Commands,
    mut movement_properties_resource: ResMut<MovementPropertiesResource>,
) {
    commands.spawn(InputManagerBundle::<MainAction> {
        action_state: ActionState::default(),
        input_map: InputMap::new([
            (InputKind::Keyboard(KeyCode::B), MainAction::BuildMenu),
            (InputKind::Mouse(MouseButton::Left), MainAction::LeftClick),
        ]),
    });
    movement_properties_resource.boost = BOOST;
    movement_properties_resource.jump = JUMP;
    movement_properties_resource.torque = TORQUE;
}

pub fn state_enter_in_game(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}

pub fn in_game(
    mut next_state: ResMut<NextState<AppState>>,
    query_action: Query<&ActionState<MainAction>>,
) {
    let action_state = &query_action.single();
    if action_state.just_pressed(MainAction::BuildMenu) {
        next_state.set(AppState::LevelEditor);
    }
}
