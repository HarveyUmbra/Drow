use avian3d::prelude::{
    Collider,
    Position,
    RigidBody,
};
use bevy::prelude::*;
use drow_nav::prelude::*;

pub fn spawn_test_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0),
        Position::from_xyz(10.0, 0.5, 10.0),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Mesh3d(mesh),
        Obstacle,
    ));
}

pub fn spawn_ground(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let lev_entity = commands.spawn_empty().id();

    commands.entity(lev_entity).insert((
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(10.0, 10.0)))),
        NavGround,
    ));
}
