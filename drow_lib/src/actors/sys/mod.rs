mod nav;
use super::{
    com::*,
    res::*,
};
use avian3d::prelude::{
    Collider,
    LockedAxes,
    Position,
    RigidBody,
};
use bevy::prelude::*;
use vleue_navigator::prelude::*;

pub fn spawn_actor_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = meshes.add(Capsule3d::new(0.5, 1.8));
    commands.spawn((
        Actor::default(),
        Target {
            ..Default::default()
        },
        Path::default(),
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
}

// Wird nicht ausgeführt?
/// Dieses System berechnet den Pfad zu einem Ziel. // A* Algoritmus
pub fn compute_path_actor(
    navmeshes: Res<Assets<NavMesh>>,
    mut query: Query<(&mut Path, &Target, &Position), Changed<Target>>,
) {
    //info!("Hay ich war Hier 1");
    for (mut path, target, position) in query.iter_mut() {
        if let Some(navmesh) = navmeshes.get(target.NavMesh.id()) {
            if let Some(new_path) = navmesh.transformed_path(position.0, target.target) {
                path.path = new_path.path
            } else {
                info!("createt not a path");
            }
        } else {
            info!("Navmesh of Handle is not da");
        }
    }
}

/// Dieses System bewegt alle Actors an einen Pfad entlang und weicht anderen // Steering/RVO
fn move_actor(//
    //query: Query<Position>,
) {
}

pub fn display_path_actor(
    query: Query<(&Transform, &Path)>,
    mut gizmos: Gizmos,
    primary_window: Single<&Window>,
) {
    let window = *primary_window;

    for (transform, path) in &query {
        if path.path.is_empty() {
            continue;
        }
        gizmos.linestrip(
            path.path.iter().map(|p| (p.clone())),
            bevy::color::palettes::css::ORANGE,
        )
    }
}

fn is_taget_okay() {}
// recast zu generierung von NavMesh? Gdot vleue_navigator
