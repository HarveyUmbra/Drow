mod com;
mod eve;
mod res;
mod sys;

use bevy::prelude::*;
use drow_core::prelude::*;
use sys::*;

use crate::actors::res::SelectedActor;

pub struct ActorsPlugin;
impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedActor>()
            .add_systems(OnEnter(AppState::Game), spawn_actor_setup)
            .add_systems(Update, giz)
            .add_observer(deslect_actor)
            .add_observer(select_actor);
    }
}
