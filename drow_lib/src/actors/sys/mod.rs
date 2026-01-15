mod nav;
use super::{
    com::*,
    res::*,
};
use avian3d::prelude::*;
use bevy::prelude::*;
use drow_nav::prelude::*;

pub fn spawn_actor_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = meshes.add(Capsule3d::new(0.5, 1.8));
    commands.spawn((
        Actor::default(),
        Navigator,
        RigidBody::Dynamic,
        Position::from_xyz(0.0, 2.0, 0.0),
        LockedAxes::new().lock_rotation_z().lock_rotation_x(),
        Collider::capsule(0.5, 1.8),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Mesh3d(mesh),
        Transform::default(),
    ));
}

/// Funktion die es ermöglich ein Actor auszuwählen.
/// ToDo: Umprogrammieren auf ein Query system hatt
pub fn select_actor(
    event: On<Pointer<Click>>,
    mut selected: ResMut<SelectedActor>,
    query: Query<Entity, With<Actor>>,
) {
    if query.contains(event.event_target()) {
        selected.0 = Some(event.event_target());
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
