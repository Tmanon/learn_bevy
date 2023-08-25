use bevy::{prelude::*, window::WindowMode::*};
use bevy_xpbd_2d::prelude::*;

pub mod actions;
pub mod camera;
pub mod game;
pub mod menu;
use actions::ActionPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use menu::MenuPlugin;

pub mod body_bundles;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            ActionPlugin,
            CameraPlugin,
            GamePlugin,
            MenuPlugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
