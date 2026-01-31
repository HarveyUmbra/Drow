pub mod com;
mod sys;
use bevy::prelude::*;
use drow_core::{
    AppState,
    GameState,
};
use sys::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_camera_setup)
            .add_observer(movement)
            .add_observer(rotate);
    }
}
