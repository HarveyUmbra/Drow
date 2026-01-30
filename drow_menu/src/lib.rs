mod logic;
pub mod style;
mod sys;
mod widgets;
use self::sys::*;
use bevy::prelude::*;
use drow_core::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(Startup, spawn_menu)
            .add_systems(Update, button_system.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), despawn_menu);
    }
}
