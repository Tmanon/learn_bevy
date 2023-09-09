use bevy::prelude::*;

use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::actions::{MovementPropertiesResource, PlayerAction};

#[derive(Component)]
pub struct MovementProperties {
    pub boost: f32,
    pub jump: f32,
    pub torque: f32,
}

impl Default for MovementProperties {
    fn default() -> Self {
        Self {
            boost: 800.0,
            jump: 200.0,
            torque: 200.0,
        }
    }
}

#[derive(Component, Default, Debug)]
pub enum BodyType {
    #[default]
    Body,
    Player,
    Wall,
}

#[derive(Component)]
pub struct Body;

#[derive(Component)]
pub struct Editing(pub bool);

#[derive(Bundle)]
pub struct BodyBundle {
    pub editing: Editing,
    pub body_type: BodyType,
    pub rigid_body: RigidBody,
    pub sprite_bundle: SpriteBundle,
    pub position: Position,
    pub collider: Collider,
}

impl Default for BodyBundle {
    fn default() -> Self {
        Self {
            editing: Editing(false),
            body_type: BodyType::Body,
            rigid_body: RigidBody::Dynamic,
            sprite_bundle: SpriteBundle {
                transform: Transform::from_scale(Vec3::new(10., 10., 0.0)),
                sprite: Sprite {
                    color: Color::ORANGE,
                    ..default()
                },
                ..default()
            },
            position: Default::default(),
            collider: Collider::cuboid(10., 10.),
        }
    }
}

impl BodyBundle {
    pub fn new(
        editing: Option<bool>,
        body_type: Option<BodyType>,
        rigid_body_type: Option<RigidBody>,
        size: Option<Vec2>,
        position: Option<Vec2>,
        color: Option<Color>,
    ) -> Self {
        Self {
            editing: Editing(editing.unwrap_or_default()),
            body_type: body_type.unwrap_or(BodyType::Body),
            rigid_body: rigid_body_type.unwrap_or_default(),
            sprite_bundle: SpriteBundle {
                transform: Transform::from_scale(Vec3::new(
                    size.unwrap_or(Vec2::splat(10.)).x,
                    size.unwrap_or(Vec2::splat(10.)).y,
                    1.0,
                )),
                sprite: Sprite {
                    color: color.unwrap_or(Color::ORANGE),
                    ..default()
                },
                ..default()
            },
            position: Position(position.unwrap_or(BodyBundle::default().position.0)),
            collider: Collider::cuboid(
                size.unwrap_or(Vec2::splat(10.)).x,
                size.unwrap_or(Vec2::splat(10.)).y,
            ),
            ..default()
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub body_bundle: BodyBundle,
    pub external_force: ExternalForce,
    pub external_torque: ExternalTorque,
    pub linear_damping: LinearDamping,
    pub angular_damping: AngularDamping,
    pub movement_properties: MovementProperties,
    pub input_manager_bundle: InputManagerBundle<PlayerAction>,
}

impl PlayerBundle {
    pub fn new(
        editing: Option<bool>,
        size: Option<Vec2>,
        position: Option<Vec2>,
        color: Option<Color>,
        movement_properties_resource: Option<Res<MovementPropertiesResource>>,
        boost: Option<f32>,
        jump: Option<f32>,
        torque: Option<f32>,
        key_boost: Option<KeyCode>,
        key_jump: Option<KeyCode>,
        key_rotate_left: Option<KeyCode>,
        key_rotate_right: Option<KeyCode>,
    ) -> Self {
        Self {
            body_bundle: BodyBundle::new(
                editing,
                Some(BodyType::Player),
                Some(RigidBody::Dynamic),
                Some(size.unwrap_or_else(|| {
                    PlayerBundle::default()
                        .body_bundle
                        .sprite_bundle
                        .transform
                        .scale
                        .truncate()
                })),
                Some(position.unwrap_or_else(|| PlayerBundle::default().body_bundle.position.0)),
                Some(color.unwrap_or(Color::PINK)),
            ),
            movement_properties: MovementProperties {
                boost: boost.unwrap_or(match &movement_properties_resource {
                    None => PlayerBundle::default().movement_properties.boost,
                    Some(i) => i.boost,
                }),
                jump: jump.unwrap_or(match &movement_properties_resource {
                    None => PlayerBundle::default().movement_properties.jump,
                    Some(i) => i.jump,
                }),
                torque: torque.unwrap_or(match &movement_properties_resource {
                    None => PlayerBundle::default().movement_properties.torque,
                    Some(i) => i.torque,
                }),
            },
            input_manager_bundle: InputManagerBundle::<PlayerAction> {
                input_map: InputMap::new([
                    (
                        key_boost.unwrap_or_else(|| KeyCode::Up),
                        PlayerAction::Boost,
                    ),
                    (
                        key_jump.unwrap_or_else(|| KeyCode::Space),
                        PlayerAction::Jump,
                    ),
                    (
                        key_rotate_left.unwrap_or_else(|| KeyCode::Left),
                        PlayerAction::RotateLeft,
                    ),
                    (
                        key_rotate_right.unwrap_or_else(|| KeyCode::Right),
                        PlayerAction::RotateRight,
                    ),
                ]),
                ..default()
            },
            ..default()
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            body_bundle: BodyBundle::new(
                None,
                Some(BodyType::Player),
                Some(RigidBody::Dynamic),
                None,
                None,
                Some(Color::PINK),
            ),
            external_force: ExternalForce::new(Vec2::ZERO).with_persistence(false),
            external_torque: ExternalTorque::new(0.0).with_persistence(false),
            linear_damping: LinearDamping(
                0.020
                    * ColliderMassProperties::new_computed(&BodyBundle::default().collider, 1.)
                        .mass
                        .0,
            ),
            angular_damping: AngularDamping(
                0.016
                    * ColliderMassProperties::new_computed(&BodyBundle::default().collider, 1.)
                        .inertia
                        .0,
            ),
            movement_properties: MovementProperties::default(),
            input_manager_bundle: InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map: InputMap::default(),
            },
        }
    }
}

#[derive(Bundle)]
pub struct WallBundle {
    pub body_bundle: BodyBundle,
}

impl WallBundle {
    pub fn new(
        editing: Option<bool>,
        size: Option<Vec2>,
        position: Option<Vec2>,
        color: Option<Color>,
    ) -> Self {
        Self {
            body_bundle: BodyBundle::new(
                editing,
                Some(BodyType::Wall),
                Some(RigidBody::Static),
                size,
                position,
                Some(color.unwrap_or(Color::WHITE)),
            ),
            ..default()
        }
    }
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            body_bundle: BodyBundle::new(
                None,
                Some(BodyType::Wall),
                Some(RigidBody::Static),
                None,
                None,
                Some(Color::WHITE),
            ),
        }
    }
}
