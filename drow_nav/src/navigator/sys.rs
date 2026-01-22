use super::*;
use avian3d::prelude::*;
use bevy::{
    ecs::query,
    prelude::*,
    window::PrimaryWindow,
};
use vleue_navigator::prelude::*;

pub fn check_navigator_ground(
    spaciel: SpatialQuery,
    navigators: Query<(&Position, &mut NavTarget), With<Navigator>>,
) {
    for (nav_position, mut target) in navigators {
        // Gibt die Positionen der Navigators
        if let Some(hit) = spaciel.cast_ray(
            nav_position.0,
            Dir3::NEG_Y,
            1.0,
            true,
            &SpatialQueryFilter::from_mask(1), // Todo Filter nur objekte mit
        ) {
            target.nav_mesh = Some(hit.entity);
            info!("hit mich richtig ?{}", hit.entity.index());
            /*
            if let Ok(manager) = navmesh_entities.get(hit.entity) {
                if let Some(nav_mesh) = navmesh_assets.get(manager.id()) {
                    target.nav_mesh = manager.id();
                } else {
                    info!("ManagedNavMesh dont have a Navmesh");
                }
            } else {
                info!("Entity "); // Todo
            }
             */
        } else {
            info!("Raycast have no found a Entity") // Todo
        }
    }
}

// Wird nicht ausgeführt?
/// Dieses System berechnet den Pfad zu einem Ziel. // A* Algoritmus
pub fn compute_path(
    mut query: Query<(&mut NavPath, &NavTarget, &Position), Changed<NavTarget>>,
    grounds: Query<&ManagedNavMesh>,
    nav_meshs: ResMut<Assets<NavMesh>>,
) {
    for (mut path, target, position) in query.iter_mut() {
        if let Some(entity) = target.nav_mesh {
            if let Ok(ground) = grounds.get(entity) {
                if let Some(navmesh) = nav_meshs.get(ground.id()) {
                    if let Some(new_path) = navmesh.transformed_path(position.0, target.target) {
                        path.path = new_path.path
                    } else {
                        info!("compute Path Failed")
                    }
                } else {
                    info!("No Navmesh")
                };
            }
        } else {
            info!("compute_path: Target dont have a Entity")
        }
    }
}

//Debug System soll Path anzeigen
pub fn display_path(
    query: Query<(&Transform, &NavPath)>,
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

pub fn handle_mouse_clicks(
    mut query: Query<&mut NavTarget>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    spatial_query: SpatialQuery,
    window: Single<&Window, With<PrimaryWindow>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = *camera_query;

        if let Some(viewport_position) = window.cursor_position() {
            if let Ok(ray) = camera.viewport_to_world(camera_transform, viewport_position) {
                // Avian Spatial Query: Findet den ersten Treffer
                if let Some(hit) = spatial_query.cast_ray(
                    ray.origin,                         // Startpunkt
                    ray.direction,                      // Richtung (muss normalisiert sein)
                    1000.0,                             // Maximale Distanz
                    true, // 'Solid': Treffer auch wenn Ursprung in Collider
                    &SpatialQueryFilter::from_mask(32), // Filter für Layer/Entities
                ) {
                    if let Ok(mut target) = query.single_mut() {
                        target.target = ray.origin + (*ray.direction * hit.distance);
                        target.nav_mesh = Some(hit.entity);
                    }
                    println!("Getroffene Entity: {:?}", hit.entity);
                }
            }
        }
    }
}
