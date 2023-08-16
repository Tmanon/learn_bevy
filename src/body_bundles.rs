use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

#[derive(Bundle)]
pub struct BodyBundle {
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
    pub body_bundle: BodyBundle,
    pub rigid_body: RigidBody,
    pub external_force: ExternalForce,
    //pub gravity: ExternalForce,
    pub external_torque: ExternalTorque,
    pub angular_damping: AngularDamping,
    pub linear_damping: LinearDamping,
    pub _p: Player,
}

#[derive(Bundle)]
pub struct WallBundle {
    pub body_bundle: BodyBundle,
    pub rigid_body: RigidBody,
    pub _w: Wall,
}

impl Default for BodyBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_scale(Vec3::new(10.0, 10.0, 1.0)),
                ..default()
            },
            position: Default::default(),
            collider: Collider::cuboid(10.0, 10.0),
        }
    }
}

impl BodyBundle {
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
            body_bundle: BodyBundle::default(),
            rigid_body: RigidBody::Dynamic,
            external_force: ExternalForce::new(Vec2::ZERO).with_persistence(false),
            //gravity: ExternalForce::new(Vec2::X * 600.0),
            external_torque: ExternalTorque::new(0.0).with_persistence(false),
            angular_damping: AngularDamping(10.0),
            linear_damping: LinearDamping(2.0),
            _p: Player,
        }
    }
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            body_bundle: BodyBundle::default(),
            rigid_body: RigidBody::Static,
            _w: Wall,
        }
    }
}
