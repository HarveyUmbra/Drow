use avian3d::prelude::{
    Collider,
    CollisionLayers,
    Position,
    RigidBody,
    Rotation,
};
use bevy::prelude::*;
use drow_core::prelude::{
    LayerMask,
    *,
};
use drow_nav::prelude::*;

pub fn spawn_test_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let list = vec![
        Vec3::new(10.0, 0.5, 10.0),
        Vec3::new(4.0, 0.5, 3.0),
        Vec3::new(-4.0, 0.5, 3.0),
    ];
    let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    for vec in list {
        commands.spawn((
            RigidBody::Static,
            Collider::cuboid(1.0, 1.0, 1.0),
            Position::from(vec),
            MeshMaterial3d(materials.add(StandardMaterial::default())),
            Mesh3d(mesh.clone()),
            Obstacle,
            DespawnOnExit(AppState::Game),
        ));
    }
    commands.trigger(SpawnActorEvent {
        position: Position::from_xyz(5.0, 2.0, 0.0),
        ..default()
    });
    commands.trigger(SpawnActorEvent {
        position: Position::from_xyz(0.0, 2.0, 0.0),
        ..default()
    });
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
        CollisionLayers::new(LayerMask::Ground, LayerMask::Actors),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(10.0, 10.0)))),
        NavGround,
        DespawnOnExit(AppState::Game),
    ));
}
