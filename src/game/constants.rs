use bevy::math::Vec2;

pub const ARENA_SIZE: Vec2 = Vec2::new(1920.0 * 0.8, 1080.0 * 0.8);
pub const ARENA_WALL: f32 = 20.0;

pub const SCALE_FACTOR: f32 = 0.0;
pub const GRAVITY: f32 = SCALE_FACTOR * 0.0;
pub const BOOST: f32 = SCALE_FACTOR * 6000000.0;
pub const JUMP: f32 = SCALE_FACTOR * 600.0;
pub const TORQUE: f32 = SCALE_FACTOR * 90000000.0;
