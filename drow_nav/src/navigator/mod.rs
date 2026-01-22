pub mod sys;

use bevy::prelude::*;

// Fügt alle Notwendigen Compenenten hinzu für einen Navigator
#[derive(Debug, Component)]
#[require(NavTarget, NavPath)]
pub struct Navigator;

#[derive(Debug, Component, Default)]
pub struct NavTarget {
    pub target: Vec3,
    pub nav_mesh: Option<Entity>,
}

#[derive(Component, Default, Reflect)]
pub struct NavPath {
    pub path: Vec<Vec3>,
}
