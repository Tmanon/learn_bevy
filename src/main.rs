use bevy::{prelude::*, window::WindowMode::*, window::WindowResolution};
use bevy_xpbd_2d::prelude::*;

pub mod actions;
pub mod camera;
pub mod game;
pub mod level_editor;
pub mod states;

use actions::ActionPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use level_editor::LevelEditorPlugin;
use states::AppState;

pub mod body_bundles;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::default().with_scale_factor_override(1.),
                    resizable: true,
                    mode: BorderlessFullscreen,
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            ActionPlugin,
            CameraPlugin,
            GamePlugin,
            LevelEditorPlugin,
        ))
        .add_state::<AppState>()
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
