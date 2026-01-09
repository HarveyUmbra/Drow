pub mod com;
mod sys;
use bevy::prelude::*;

use sys::*;

pub struct TestPlugin;
impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ground, spawn_test_setup));
    }
}
