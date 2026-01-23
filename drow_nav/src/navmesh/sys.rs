use super::NavGround;
use avian3d::prelude::*;
use bevy::{
    color::palettes::css::RED,
    prelude::*,
};
use vleue_navigator::prelude::*;

use drow_core::prelude::{
    LayerMask,
    *,
};

pub fn setup_ground(trigger: On<Add, NavGround>, mut commands: Commands) {
    let nav_entity = commands.spawn_empty().id();
    let id = nav_entity.index().index() as u128;

    let mesh_c = &Mesh::from(Plane3d::new(Vec3::Z, Vec2::new(10.0, 10.0)));
    let mesh = &Mesh::from(Plane3d::new(Vec3::Y, Vec2::new(10.0, 10.0)));

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
        Collider::trimesh_from_mesh(mesh_c)
            .expect("NavMesh konnte nicht in Collider umgewandelt werden"),
        CollisionLayers::new(LayerMask::Navmesh, LayerMask::None),
    ));

    commands.entity(trigger.entity).add_child(nav_entity);
}

pub fn despawn_ground(trigger: On<Remove, NavGround>, mut commands: Commands) {
    commands.entity(trigger.entity).despawn_children();
}
