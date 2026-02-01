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
