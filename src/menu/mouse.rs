use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;

use crate::menu::rect_menu::BodyList;

pub fn mouse_system(
    mut query: Query<&mut Position, With<BodyList>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    //camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    //let (camera, camera_transform) = camera_q.single();
    //let window = if let RenderTarget::Window(Entity) = camera.target {
    //    windows.get(RenderTarget::Window(id)).unwrap()
    //} else {
    //    windows.get_primary().unwrap()
    //};
    //if let Some(world_position) = window
    //    .cursor_position()
    //    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    //    .map(|ray| ray.origin.truncate())
    //{
    //    eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    //}

    let cursor = windows.single().cursor_position().unwrap_or_default();
    for mut position in &mut query {
        //position.0 = cursor.copysign(Vec2::NEG_Y).add(Vec2::Y * 500.);
        position.0 = cursor;
    }
}
