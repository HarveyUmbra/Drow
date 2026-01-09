pub mod actions;
mod actors;
mod camera;
mod input;
pub mod prelude;
mod test;

use avian3d::prelude::*;
use bevy::app::{
    PluginGroup,
    PluginGroupBuilder,
};

use crate::{
    actors::ActorsPlugin,
    camera::CameraPlugin,
    input::InputPlugin,
    test::{
        TestPlugin,
        com::Obstacle,
    },
};

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PhysicsPickingPlugin) // EXTERN // EXTERN: Generation NavMesh
            .add(InputPlugin)
            .add(CameraPlugin)
            .add(ActorsPlugin)
            .add(TestPlugin)
    }
}
