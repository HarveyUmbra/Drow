use avian3d::prelude::*;
use bevy::prelude::*;
use drow_core::prelude::LayerMask;
use drow_nav::prelude::Navigator;

#[derive(Component)]
#[require(Navigator, LockedAxes, RigidBody)]
pub struct Actor(pub String);

impl Default for Actor {
    fn default() -> Self {
        Actor("Actor".to_string())
    }
}

#[derive(Bundle)]
pub struct ActorBundle {
    pub actor: Actor,
    pub rigid_body: RigidBody,
    pub position: Position,
    pub locked_axes: LockedAxes,
    pub collider: Collider,
    pub collison_layer: CollisionLayers,
}

impl Default for ActorBundle {
    fn default() -> Self {
        return ActorBundle {
            actor: Actor::default(),
            rigid_body: RigidBody::Dynamic,
            position: Position::default(),
            locked_axes: LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            collider: Collider::compound(vec![(
                Vec3::new(0.0, 1.4, 0.0),
                Quat::IDENTITY,
                Collider::capsule(0.5, 1.8),
            )]),
            collison_layer: CollisionLayers::new(
                LayerMask::Actors,
                [LayerMask::Ground, LayerMask::Actors],
            ),
        };
    }
}

/*
#[derive(Component)]
pub struct Fae;
#[derive(Component)]
pub struct Ghost;
#[derive(Component)]
pub struct Ghoul;
#[derive(Component)]
pub struct Hollow;
#[derive(Component)]
pub struct Infernal;
#[derive(Component)]
pub struct Mortal;
#[derive(Component)]
pub struct Queen;
#[derive(Component)]
pub struct Vampire;
#[derive(Component)]
pub struct Werewolf;
#[derive(Component)]
pub struct Witch;

struct attributes {
    Hot: i8,
    Dark: i8,
    Cold: i8,
}

#[derive(Component)]
struct Health(i32);
 */
