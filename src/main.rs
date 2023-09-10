use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*, window::WindowMode::*, window::WindowResolution};
use bevy_xpbd_2d::prelude::*;

pub mod actions;
pub mod camera;
pub mod game;
pub mod level_editor;
pub mod scene;
pub mod states;

use actions::ActionPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use level_editor::LevelEditorPlugin;
use scene::ScenePlugin;
use states::AppState;

pub mod body_bundles;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::default().with_scale_factor_override(1.),
                        resizable: true,
                        mode: BorderlessFullscreen,
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                }),
            PhysicsPlugins::default(),
            ActionPlugin,
            CameraPlugin,
            GamePlugin,
            LevelEditorPlugin,
            ScenePlugin,
        ))
        .add_state::<AppState>()
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
