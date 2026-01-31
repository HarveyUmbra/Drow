mod com;
mod eve;
use bevy::prelude::*;
// This Stat
#[derive(States, Clone, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub enum AppState {
    Game, // The Game Loop
    #[default]
    Menu, // Main Menu Loop
    Load, // Load Assets
}

#[derive(SubStates, Clone, Debug, Default, Eq, Hash, PartialEq)]
#[source(AppState = AppState::Game)]
pub enum GameState {
    #[default]
    Run,
    Stop,
}

pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<State<AppState>>()
            .insert_state(AppState::default())
            .add_sub_state::<GameState>();
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
