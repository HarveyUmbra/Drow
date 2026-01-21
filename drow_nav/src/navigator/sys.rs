use super::*;
use avian3d::prelude::*;
use bevy::prelude::*;
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
            &SpatialQueryFilter::default(), // Todo Filter nur objekte mit
        ) {
            target.nav_mesh = Some(hit.entity);
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
