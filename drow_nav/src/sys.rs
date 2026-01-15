use super::com::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use vleue_navigator::prelude::*;

// Wird nicht ausgeführt?
/// Dieses System berechnet den Pfad zu einem Ziel. // A* Algoritmus
pub fn compute_path(
    navmeshes: Res<Assets<NavMesh>>,
    mut query: Query<(&mut Path, &Target, &Position), Changed<Target>>,
) {
    //info!("Hay ich war Hier 1");
    for (mut path, target, position) in query.iter_mut() {
        if let Some(navmesh) = navmeshes.get(target.nav_mesh.id()) {
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

pub fn display_path(
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
