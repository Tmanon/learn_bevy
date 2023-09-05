use bevy::render::camera::RenderTarget;
use bevy::window::WindowRef;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;

use crate::camera::MainCamera;
use crate::level_editor::rect_menu::BodyList;

pub fn place_bodies(
    mut query: Query<&mut Position, With<BodyList>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
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
    //{
    //    eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    //}

    //let cursor = windows.single().cursor_position().unwrap_or_default();
    for mut position in &mut query {
        position.0 = world_position.unwrap_or_default();
        //position.0 = cursor.copysign(Vec2::NEG_Y) + (Vec2::Y * 500.);
        //position.0 = cursor;
    }
}
