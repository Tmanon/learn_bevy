use bevy::render::camera::RenderTarget;
use bevy::window::WindowRef;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::actions::MainAction;
use crate::body_bundles::Editing;
use crate::camera::MainCamera;
use crate::states::AppState;

pub fn place_bodies(
    mut query: Query<(&mut Position, &Editing)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut next_state: ResMut<NextState<AppState>>,
    actions_query: Query<&ActionState<MainAction>>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = if let RenderTarget::Window(WindowRef::Entity(entity)) = camera.target {
        windows.get(entity).unwrap()
    } else {
        windows.get_single().unwrap()
    };
    let world_position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
    for (mut position, editing) in &mut query {
        if editing.0 {
            position.0 = world_position.unwrap_or_default();
        }
    }

    if actions_query.single().released(MainAction::LeftClick) {
        next_state.set(AppState::LevelEditor);
    }
}

pub fn place_bodies_exit(mut query: Query<&mut Editing>) {
    for mut editing in &mut query {
        *editing = Editing(false);
    }
}
