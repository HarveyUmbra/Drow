mod eve;
pub mod prelude {
    use avian3d::prelude::*;

    pub use super::eve::*;

    #[derive(PhysicsLayer, Default)]
    pub enum LayerMask {
        #[default]
        None, // Nie vergeben für ein membership
        Ground,
        Actors,
        Navmesh,
    }
}
