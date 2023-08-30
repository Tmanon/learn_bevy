use bevy::prelude::*;

use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::actions::{MovementPropertiesResource, PlayerAction};

//So I know how to this manually by copying the code for each button and component bundle, but I wonder how I can reuse the code.
//This is a pseudo code for what I want to do
//```
//#[derive(Bundle)]
//struct BundleOne{...}
//...BundleTwo...
//...BundleThree...
//
//impl Default for BundleOne {...}
//impl Default for BundleTwo {...}
//impl Default for BundleThree {...}
//
//#[derive(BundleList)]
//enum Bundles {
//    BundleOne,
//    BundleTwo,
//    BundleThree,
//}
//
//fn bundle_buttons(mut commands: Commands) {
//    for bundle in Bundles::list {
//        commands.spawn(Button(name: bundle));
//    }
//}
//
//fn button_system(mut commands: Commands, query: Query<Button>) {
//    for ( button) in &query{
//        if button::pressed{
//            commands.spawn(button::name::default());
//        }
//    }
//}
//
//#[derive(Component)]
//pub enum Bodies {
//    Body(BodyBundle),
//    Wall(WallBundle),
//    Player(PlayerBundle),
//}

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

#[derive(Component)]
pub struct Body;

#[derive(Bundle)]
pub struct BodyBundle {
    pub rigid_body: RigidBody,
    pub sprite_bundle: SpriteBundle,
    pub position: Position,
    pub collider: Collider,
    pub _b: Body,
}

impl Default for BodyBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            sprite_bundle: SpriteBundle {
                transform: Transform::from_scale(Vec3::new(10., 10., 0.0)),
                ..default()
            },
            position: Default::default(),
            collider: Collider::cuboid(10., 10.),
            _b: Body,
        }
    }
}

impl BodyBundle {
    pub fn new(
        rigid_body_type: Option<RigidBody>,
        size: Option<Vec2>,
        position: Option<Vec2>,
    ) -> Self {
        Self {
            rigid_body: rigid_body_type.unwrap_or_default(),
            sprite_bundle: SpriteBundle {
                transform: Transform::from_scale(Vec3::new(
                    size.unwrap_or(Vec2::splat(10.)).x,
                    size.unwrap_or(Vec2::splat(10.)).y,
                    1.0,
                )),
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
    pub _p: Player,
}

impl PlayerBundle {
    pub fn new(
        size: Option<Vec2>,
        position: Option<Vec2>,
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
                None,
                Some(size.unwrap_or_else(|| {
                    PlayerBundle::default()
                        .body_bundle
                        .sprite_bundle
                        .transform
                        .scale
                        .truncate()
                })),
                Some(position.unwrap_or_else(|| PlayerBundle::default().body_bundle.position.0)),
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
            body_bundle: BodyBundle::default(),
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
            _p: Player,
        }
    }
}

#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
pub struct WallBundle {
    pub body_bundle: BodyBundle,
    pub _w: Wall,
}

impl WallBundle {
    pub fn new(size: Vec2, position: Vec2) -> Self {
        Self {
            body_bundle: BodyBundle::new(Some(RigidBody::Static), Some(size), Some(position)),
            ..default()
        }
    }
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            body_bundle: BodyBundle::new(Some(RigidBody::Static), None, None),
            _w: Wall,
        }
    }
}
