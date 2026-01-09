mod sys;
use bevy::prelude::*;
use sys::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                controler_wasd, //
                controler_eq,
            ),
        );
    }
}
