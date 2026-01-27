pub mod sys;
use avian3d::prelude::*;
use bevy::prelude::*;

// Fügt alle Notwendigen Compenenten hinzu für einen Navigator
#[derive(Debug, Component, Default)]
#[require(NavTarget, NavPath, Rotation, Position)]
pub struct Navigator;

#[derive(Debug, Component, Default, Reflect)]
pub struct NavTarget {
    pub target: Vec3,
    pub nav_mesh: Option<Entity>,
}

#[derive(Component, Default, Reflect)]
pub struct NavPath {
    pub path: Vec<Vec3>,
}
