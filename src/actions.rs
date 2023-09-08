use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::body_bundles::MovementProperties;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementPropertiesResource::default())
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_plugins(InputManagerPlugin::<MainAction>::default())
            .add_systems(Update, movement_system);
    }
}

#[derive(Resource)]
pub struct MovementPropertiesResource {
    pub boost: f32,
    pub jump: f32,
    pub torque: f32,
}

impl Default for MovementPropertiesResource {
    fn default() -> Self {
        Self {
            boost: MovementProperties::default().boost,
            jump: MovementProperties::default().jump,
            torque: MovementProperties::default().torque,
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MainAction {
    BuildMenu,
    LeftClick,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Boost,
    Jump,
    RotateLeft,
    RotateRight,
}

fn movement_system(
    mut physics_query: Query<(
        &ColliderMassProperties,
        &MovementProperties,
        &mut ExternalForce,
        &mut LinearVelocity,
        &mut ExternalTorque,
        &Rotation,
    )>,
    action_query: Query<&ActionState<PlayerAction>>,
) {
    for action_state in &action_query {
        for (
            collider_mass_properties,
            movement_properties,
            mut external_force,
            mut linear_velocity,
            mut external_torque,
            rotation,
        ) in &mut physics_query
        {
            if action_state.pressed(PlayerAction::Boost) {
                external_force.x =
                    rotation.cos() * movement_properties.boost * collider_mass_properties.mass.0;
                external_force.y =
                    rotation.sin() * movement_properties.boost * collider_mass_properties.mass.0;
            }
            if action_state.just_pressed(PlayerAction::Jump) {
                linear_velocity.x += rotation.cos() * movement_properties.jump;
                linear_velocity.y += rotation.sin() * movement_properties.jump;
            }
            if action_state.pressed(PlayerAction::RotateLeft) {
                external_torque
                    .apply_torque(movement_properties.torque * collider_mass_properties.inertia.0);
            }
            if action_state.pressed(PlayerAction::RotateRight) {
                external_torque
                    .apply_torque(-movement_properties.torque * collider_mass_properties.inertia.0);
            }
        }
    }
}
