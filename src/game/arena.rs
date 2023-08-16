use bevy::prelude::*;

use crate::body_bundles::*;
use crate::constants::*;

pub fn spawn_arena(mut commands: Commands) {
    let half_size: Vec2 = Vec2::new(ARENA_SIZE.x * 0.5, ARENA_SIZE.y * 0.5);
    commands.spawn(WallBundle {
        body_bundle: BodyBundle::new(Vec2::new(ARENA_SIZE.x, ARENA_WALL), Vec2::Y * half_size.y),
        ..default()
    });
    commands.spawn(WallBundle {
        body_bundle: BodyBundle::new(
            Vec2::new(ARENA_SIZE.x, ARENA_WALL),
            Vec2::NEG_Y * half_size.y,
        ),
        ..default()
    });
    commands.spawn(WallBundle {
        body_bundle: BodyBundle::new(Vec2::new(ARENA_WALL, ARENA_SIZE.y), Vec2::X * half_size.x),
        ..default()
    });
    commands.spawn(WallBundle {
        body_bundle: BodyBundle::new(
            Vec2::new(ARENA_WALL, ARENA_SIZE.y),
            Vec2::NEG_X * half_size.x,
        ),
        ..default()
    });
}
