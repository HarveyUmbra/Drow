use super::{
    com::*,
    res::*,
};
use avian3d::prelude::*;
use bevy::{
    ecs::query,
    prelude::*,
};

use drow_core::prelude::{
    LayerMask,
    *,
};
use drow_nav::prelude::*;

pub fn spawn_actor_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = Mesh::from(Capsule3d::new(0.5, 1.8)).translated_by(Vec3::new(0.0, 1.4, 0.0));
    let mesh = meshes.add(mesh);
    commands.spawn((
        Name::new("Actor"),
        Actor::default(),
        Navigator,
        RigidBody::Dynamic,
        Position::from_xyz(0.0, 2.0, 0.0),
        LockedAxes::new().lock_rotation_z().lock_rotation_x(),
        Collider::compound(vec![(
            Vec3::new(0.0, 1.4, 0.0),
            Quat::IDENTITY,
            Collider::capsule(0.5, 1.8),
        )]),
        CollisionLayers::new(LayerMask::Actors, [LayerMask::Ground, LayerMask::Actors]),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Mesh3d(mesh),
        Transform::default(),
    ));
}

/// Funktion die es ermöglich ein Actor auszuwählen.
/// ToDo: Umprogrammieren auf ein Query system hatt
pub fn select_actor(
    event: On<SelectActorRequest>,
    mut selected: ResMut<SelectedActor>,
    query: Query<Entity, With<Actor>>,
) {
    if query.contains(event.entity) {
        selected.0 = Some(event.entity);
    }
}

pub fn giz(mut gizmos: Gizmos, query: Query<(&Position, &Rotation), With<Actor>>) {
    //gizmos.arrow(start, end, bevy::color::palettes::css::AZURE)
    for (position, rotation) in query.iter() {
        gizmos.arrow(
            position.0, //
            position.0
                + Vec3::NEG_Z
                    .rotate_axis(rotation.0.to_axis_angle().0, rotation.0.to_axis_angle().1)
                    * 2.0,
            bevy::color::palettes::css::AZURE,
        );
    }
}

/*
pub fn set_target_actor(
    event: On<Pointer<Click>>,
    selected: Res<SelectedActor>,
    mut query: Query<&mut Target, With<Actor>>,
) {
    if let Some(entity) = selected.0
        && !query.contains(event.event_target())
    {
        if let Some(hit_position) = event.hit.position {
            if let Ok(mut target) = query.get_mut(entity) {
                target.target = hit_position;
            }
        }
    }
} */
