mod com;
mod eve;
mod res;
mod sys;

use bevy::prelude::*;
use sys::*;

use crate::actors::res::SelectedActor;

pub struct ActorsPlugin;
impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedActor>()
            .add_systems(Startup, spawn_actor_setup)
            .add_observer(select_actor);
        //.add_observer(set_target_actor);
    }
}
