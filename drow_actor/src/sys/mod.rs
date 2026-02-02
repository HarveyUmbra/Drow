use super::com::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use drow_core::prelude::*;

pub fn spawn_actor(
    event: On<SpawnActorEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = Mesh::from(Capsule3d::new(0.5, 1.8)).translated_by(Vec3::new(0.0, 1.4, 0.0));
    let mesh = meshes.add(mesh);
    commands.spawn((
        Name::new("Actor"),
        ActorBundle {
            actor: Actor::default(),
            position: event.position,
            ..default()
        },
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Mesh3d(mesh.clone()),
        Navigator,
    ));
}

/// Funktion die es ermöglich ein Actor auszuwählen.
/// ToDo: Umprogrammieren auf ein Query system hatt
pub fn select_actor(
    event: On<SelectActorsRequest>,
    mut commands: Commands,
    query: Query<Entity, (With<Actor>, Without<Selected>)>,
) {
    for entity in query.iter_many(&event.entities) {
        commands.entity(entity).insert(Selected);
    }
}

pub fn deslect_actor(
    event: On<DeselectActorsRequest>,
    mut commands: Commands,
    query: Query<Entity, With<Selected>>,
) {
    for entity in query
        .iter()
        .filter(|e| !event.exceptions_entities.contains(&e))
    {
        commands.entity(entity).remove::<Selected>();
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
