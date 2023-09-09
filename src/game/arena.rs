use bevy::prelude::*;

use crate::body_bundles::*;
use crate::game::constants::*;

pub fn spawn_arena(mut commands: Commands) {
    let half_size: Vec2 = Vec2::new(ARENA_SIZE.x * 0.5, ARENA_SIZE.y * 0.5);
    commands.spawn(WallBundle::new(
        None,
        Some(Vec2::new(ARENA_SIZE.x, ARENA_WALL)),
        Some(Vec2::Y * half_size.y - ARENA_WALL * 0.5),
        None,
    ));
    commands.spawn(WallBundle::new(
        None,
        Some(Vec2::new(ARENA_SIZE.x, ARENA_WALL)),
        Some(Vec2::NEG_Y * half_size.y + ARENA_WALL * 0.5),
        None,
    ));
    commands.spawn(WallBundle::new(
        None,
        Some(Vec2::new(ARENA_WALL, ARENA_SIZE.y)),
        Some(Vec2::X * half_size.x - ARENA_WALL * 0.5),
        None,
    ));
    commands.spawn(WallBundle::new(
        None,
        Some(Vec2::new(ARENA_WALL, ARENA_SIZE.y)),
        Some(Vec2::NEG_X * half_size.x + ARENA_WALL * 0.5),
        None,
    ));
}
