use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub const SCALE_FACTOR: f32 = 10.0;
pub const ARENA_SIZE: Vec2 = Vec2::new(800.0, 600.0);
pub const ARENA_WALL: f32 = 20.0;

#[derive(Bundle)]
pub struct Body {
    sprite_bundle: SpriteBundle,
    position: Position,
    collider: Collider,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub body: Body,
    pub rigid_body: RigidBody,
    pub _p: Player,
}

#[derive(Bundle)]
pub struct WallBundle {
    pub body: Body,
    pub rigid_body: RigidBody,
    pub _w: Wall,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_scale(Vec3::new(SCALE_FACTOR, SCALE_FACTOR, 1.0)),
                ..default()
            },
            position: Default::default(),
            collider: Collider::cuboid(SCALE_FACTOR, SCALE_FACTOR),
        }
    }
}

impl Body {
    pub fn new(size: Vec2, position: Vec2) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_scale(Vec3::new(size.x, size.y, 1.0)),
                ..default()
            },
            position: Position(position),
            collider: Collider::cuboid(size.x, size.y),
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            body: Body::default(),
            rigid_body: RigidBody::Dynamic,
            _p: Player,
        }
    }
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            body: Body::default(),
            rigid_body: RigidBody::Static,
            _w: Wall,
        }
    }
}
