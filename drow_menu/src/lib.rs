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
        app.add_systems(Startup, spawn_menu)
            .add_systems(Update, button_system)
            .add_systems(OnEnter(AppState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Stop), spawn_stop_menu);
    }
}
