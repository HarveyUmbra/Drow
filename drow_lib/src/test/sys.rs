use avian3d::prelude::{
    Collider,
    Position,
    RigidBody,
};
use bevy::{
    color::palettes::css::RED,
    prelude::*,
};
use vleue_navigator::prelude::*;

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
        super::com::Obstacle,
    ));
}

pub fn spawn_ground(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let lev_entity = commands.spawn_empty().id();
    let nav_entity = commands.spawn_empty().id();
    let id = nav_entity.index() as u128;

    let mesh = &Mesh::from(Plane3d::new(Vec3::Y, Vec2::new(10.0, 10.0)));

    commands
        .entity(lev_entity)
        .insert((
            RigidBody::Static,
            Collider::half_space(Vec3::Y),
            MeshMaterial3d(materials.add(StandardMaterial::default())),
            Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(10.0, 10.0)))),
        ))
        .add_child(nav_entity);

    commands.entity(nav_entity).insert((
        Transform::from_rotation(Quat::from_rotation_x(90.0_f32.to_radians())),
        NavMeshSettings {
            fixed: Triangulation::from_mesh(
                NavMesh::from_bevy_mesh(mesh).unwrap().get().as_ref(),
                0,
            ),
            ..default()
        },
        NavMeshUpdateMode::Direct,
        NavMeshDebug(RED.into()),
        ManagedNavMesh::from_id(id),
    ));
}
