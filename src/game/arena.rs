use bevy::prelude::*;

use crate::body_bundles::*;
use crate::game::constants::*;

pub fn spawn_arena(mut commands: Commands) {
    let half_size: Vec2 = Vec2::new(ARENA_SIZE.x * 0.5, ARENA_SIZE.y * 0.5);
    commands.spawn(WallBundle::new(
        Vec2::new(ARENA_SIZE.x, ARENA_WALL),
        Vec2::Y * half_size.y,
    ));
    commands.spawn(WallBundle::new(
        Vec2::new(ARENA_SIZE.x, ARENA_WALL),
        Vec2::NEG_Y * half_size.y,
    ));
    commands.spawn(WallBundle::new(
        Vec2::new(ARENA_WALL, ARENA_SIZE.y),
        Vec2::X * half_size.x,
    ));
    commands.spawn(WallBundle::new(
        Vec2::new(ARENA_WALL, ARENA_SIZE.y),
        Vec2::NEG_X * half_size.x,
    ));
}
