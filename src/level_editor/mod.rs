use bevy::prelude::*;

mod menu_bundles;
mod place_bodies;
mod rect_menu;

use crate::level_editor::place_bodies::*;
use crate::level_editor::rect_menu::*;
use crate::states::AppState;

pub struct LevelEditorPlugin;

impl Plugin for LevelEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), rect_menu_setup)
            .add_systems(
                Update,
                (button_system, place_bodies).run_if(in_state(AppState::MainMenu)),
            );
    }
}
