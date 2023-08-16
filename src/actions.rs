use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementProperties::default())
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Update, action_system);
    }
}

#[derive(Resource)]
pub struct MovementProperties {
    pub boost: f32,
    pub jump: f32,
    pub torque: f32,
}

impl Default for MovementProperties {
    fn default() -> Self {
        Self {
            boost: 6000000.0,
            jump: 600.0,
            torque: 90000000.0,
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Boost,
    Jump,
    RotateLeft,
    RotateRight,
}

fn action_system(
    movement_propertes: Res<MovementProperties>,
    mut physics_query: Query<(
        &mut ExternalForce,
        &mut LinearVelocity,
        &mut ExternalTorque,
        &Rotation,
    )>,
    action_query: Query<&ActionState<Action>>,
) {
    for action_state in &action_query {
        for (mut external_force, mut linear_velocity, mut external_torque, rotation) in
            &mut physics_query
        {
            if action_state.pressed(Action::Boost) {
                external_force.x = rotation.cos() * movement_propertes.boost;
                external_force.y = rotation.sin() * movement_propertes.boost;
            }
            if action_state.just_pressed(Action::Jump) {
                linear_velocity.x += rotation.cos() * movement_propertes.jump;
                linear_velocity.y += rotation.sin() * movement_propertes.jump;
            }
            if action_state.pressed(Action::RotateLeft) {
                external_torque.apply_torque(movement_propertes.torque);
            }
            if action_state.pressed(Action::RotateRight) {
                external_torque.apply_torque(-movement_propertes.torque);
            }
        }
    }
}
