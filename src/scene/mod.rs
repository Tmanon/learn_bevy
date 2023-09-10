use bevy::prelude::*;

use crate::{actions::MovementPropertiesResource, body_bundles::BodyType, states::AppState};

use self::scene::*;

mod scene;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), save_scene_system)
            .register_type::<BodyType>()
            .register_type::<MovementPropertiesResource>();
    }
}
