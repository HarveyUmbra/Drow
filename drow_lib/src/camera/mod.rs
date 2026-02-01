pub mod com;
mod sys;
use bevy::prelude::*;
use drow_core::prelude::*;
use sys::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_camera_setup);
    }
}
