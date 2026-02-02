mod com;
mod eve;
pub mod prelude;
mod res;
mod sys;

use bevy::prelude::*;
use drow_core::prelude::*;
use sys::*;

pub struct ActorsPlugin;
impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, giz)
            .add_observer(spawn_actor)
            .add_observer(deslect_actor)
            .add_observer(select_actor);
    }
}
