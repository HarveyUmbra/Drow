use super::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use vleue_navigator::prelude::*;

use drow_core::prelude::*;

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
                    if navmesh.transformed_is_in_mesh(position.0) {
                        info!("position ist in Mesh")
                    } else {
                        info!("position ist nicht im Mesh")
                    }
                    if navmesh.transformed_is_in_mesh(target.target) {
                        info!("target   ist in Mesh")
                    } else {
                        info!("target ist nicht im Mesh")
                    }
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
    query: Query<(&Position, &NavPath)>,
    mut gizmos: Gizmos,
    primary_window: Single<&Window>,
) {
    let window = *primary_window;

    for (position, path) in &query {
        if path.path.is_empty() {
            continue;
        }
        gizmos.linestrip(
            path.path.iter().map(|p| (p.clone())),
            bevy::color::palettes::css::ORANGE,
        )
    }
}

pub fn change_target(event: On<ChangeTargetRequest>, mut query: Query<&mut NavTarget>) {
    if let Ok(mut target) = query.single_mut() {
        target.target = event.target;
        target.nav_mesh = Some(event.entity);
    }
}
