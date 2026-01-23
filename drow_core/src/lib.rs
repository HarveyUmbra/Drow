mod eve;
pub mod prelude {
    use avian3d::prelude::*;

    pub use super::eve::{
        MoveRequest,
        RotateRequest,
        SelectActorRequest,
    };

    #[derive(PhysicsLayer, Default)]
    pub enum LayerMask {
        #[default]
        None, // Nie vergeben für ein membership
        Ground,
        Actors,
        Navmesh,
    }
}
