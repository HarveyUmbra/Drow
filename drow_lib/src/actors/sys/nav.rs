use super::super::com::Actor;
use avian3d::prelude::*;
use bevy::prelude::*;

fn check_actor_ground(
    mut commands: Commands,
    actors: Query<(Entity, &Position), With<Actor>>,
    spatial_query: SpatialQuery,
) {
    for (entity, position) in actors {
        if let Some(hit) = spatial_query.cast_ray(
            position.0,
            Dir3::NEG_Y,
            0.5,
            true,
            &SpatialQueryFilter::from_excluded_entities([entity]),
        ) {
            if let Ok(get_entity) = commands.get_entity(entity) {}
            let a = hit.entity;
        } else {
            info!("Kein Hit")
        }
    }
}
