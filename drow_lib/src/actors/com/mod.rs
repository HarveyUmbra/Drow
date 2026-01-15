use bevy::prelude::*;
use vleue_navigator::{
    NavMesh,
    TransformedPath,
};

#[derive(Component)]
pub struct Actor(pub String);
impl Default for Actor {
    fn default() -> Self {
        Actor("sda".to_string())
    }
}

/*
#[derive(Component, Default, Reflect)]
pub struct Target {
    pub target: Vec3,
    pub NavMesh: Handle<NavMesh>,
}


#[derive(Event)]
pub struct FixTarget(pub Entity);

#[derive(Component, Default, Reflect)]
pub struct Path {
    pub path: Vec<Vec3>,
}
 */

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
