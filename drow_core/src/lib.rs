mod com;
mod eve;
use bevy::prelude::*;
// This Stat
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum GameState {
    #[default]
    Game, // The Game Loop
    Menu, // Main Menu Loop
    Load, // Load Assets
    Stop, // Stop the Game
    Quit, //
}

pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<State<GameState>>()
            .insert_state(GameState::default());
    }
}

pub mod prelude {
    use avian3d::prelude::*;

    pub use super::{
        com::*,
        eve::*,
        *,
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
