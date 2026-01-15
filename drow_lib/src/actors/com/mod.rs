use bevy::prelude::*;

#[derive(Component)]
pub struct Actor(pub String);
impl Default for Actor {
    fn default() -> Self {
        Actor("sda".to_string())
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
