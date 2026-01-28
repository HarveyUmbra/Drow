pub mod prelude {
    pub use super::InputPlugin;
}
mod sys;

use self::sys::*;
use bevy::prelude::*;
use drow_core::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                controler_wasd, //
                controler_eq,
                controler_click,
            )
                .run_if(in_state(GameState::Game)),
        );
    }
}
