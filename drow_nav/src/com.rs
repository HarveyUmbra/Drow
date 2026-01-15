use bevy::prelude::*;
use vleue_navigator::NavMesh;

#[derive(Debug, Event)]
pub struct Change_Event(Vec3);

// Fügt alle Notwendigen Compenenten hinzu für
#[derive(Debug, Component)]
#[require(Target, Path)]
pub struct Navigator;

#[derive(Debug, Component, Default)]
pub struct Target {
    pub target: Vec3,
    pub nav_mesh: Handle<NavMesh>,
}

#[derive(Component, Default, Reflect)]
pub struct Path {
    pub path: Vec<Vec3>,
}

#[derive(Component)]
pub struct Obstacle;
