use bevy::render::camera::RenderTarget;
use bevy::window::WindowRef;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::actions::MainAction;
use crate::body_bundles::{BodyBundle, BodyType, Editing, PlayerBundle, WallBundle};
use crate::camera::MainCamera;
use crate::level_editor::PlaceBodiesReleasedOnce;
use crate::states::AppState;

pub fn place_bodies(
    mut commands: Commands,
    mut query: Query<(Entity, &BodyType, &mut Position, &mut Editing)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut next_state: ResMut<NextState<AppState>>,
    actions_query: Query<&ActionState<MainAction>>,
    mut released_once: ResMut<PlaceBodiesReleasedOnce>,
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
    for (_entity, _body_type, mut position, editing) in &mut query {
        if editing.0 {
            position.0 = world_position.unwrap_or_default();
        }
    }

    if actions_query.single().just_released(MainAction::LeftClick) {
        if released_once.0 {
            println!("released_once = true");
            for (entity, body_type, _position, mut editing) in &mut query {
                if editing.0 {
                    println!("entity {:#?} is type {:#?}", entity, body_type);
                    *editing = Editing(false);
                    println!("set editing false for entity id: {:#?}", entity);
                    match body_type {
                        BodyType::Body => {
                            commands.spawn(BodyBundle::new(
                                Some(true),
                                None,
                                None,
                                None,
                                None,
                                None,
                            ));
                        }
                        BodyType::Player => {
                            commands.spawn(PlayerBundle::new(
                                Some(true),
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                            ));
                        }
                        BodyType::Wall => {
                            commands.spawn(WallBundle::new(Some(true), None, None, None));
                        }
                    }
                }
            }
        } else {
            released_once.0 = true;
        }
    }

    if actions_query.single().just_released(MainAction::BuildMenu) {
        for (entity, _body_type, _position, editing) in &mut query {
            if editing.0 {
                commands.entity(entity).despawn();
            }
        }
        next_state.set(AppState::LevelEditor);
    }
}

pub fn place_bodies_exit(
    mut query: Query<&mut Editing>,
    mut released_once: ResMut<PlaceBodiesReleasedOnce>,
) {
    released_once.0 = false;
    for mut editing in &mut query {
        *editing = Editing(false);
    }
}
