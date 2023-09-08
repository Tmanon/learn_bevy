use bevy::prelude::*;

use crate::actions::MovementPropertiesResource;
use crate::body_bundles::PlayerBundle;

pub fn players(
    mut commands: Commands,
    movement_properties_resource: Res<MovementPropertiesResource>,
) {
    commands.spawn(PlayerBundle::new(
        None,
        Some(Vec2::new(60., 80.)),
        //Some(Vec2::ZERO),
        None,
        Some(movement_properties_resource),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ));
}
