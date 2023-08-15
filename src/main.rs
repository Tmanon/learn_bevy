use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub mod arena;
use arena::*;
pub mod camera;
use camera::*;
mod body_bundles;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            SpawnArena,
            CameraPlugin,
        ))
        .run();
}
