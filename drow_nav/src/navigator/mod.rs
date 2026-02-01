pub mod sys;
use bevy::prelude::*;

#[derive(Debug, Component, Default, Reflect)]
pub struct NavTarget {
    pub target: Vec3,
    pub nav_mesh: Option<Entity>,
}

#[derive(Component, Default, Reflect)]
pub struct NavPath {
    pub path: Vec<Vec3>,
}
