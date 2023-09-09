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
        app.add_systems(OnEnter(AppState::LevelEditor), rect_menu_setup)
            .add_systems(OnExit(AppState::LevelEditor), rect_menu_despawn)
            .add_systems(
                Update,
                (menu_system, button_system).run_if(in_state(AppState::LevelEditor)),
            )
            .add_systems(Update, place_bodies.run_if(in_state(AppState::PlaceBodies)))
            .add_systems(OnExit(AppState::PlaceBodies), place_bodies_exit)
            .insert_resource(PlaceBodiesReleasedOnce(false));
    }
}

#[derive(Resource)]
pub struct PlaceBodiesReleasedOnce(bool);
