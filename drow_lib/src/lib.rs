mod actors;
mod camera;
pub mod prelude;
mod test;

use avian3d::prelude::*;
use bevy::app::{
    PluginGroup,
    PluginGroupBuilder,
};
use drow_core::prelude::*;
use drow_input::prelude::*;
use drow_nav::prelude::*;

use crate::{
    actors::ActorsPlugin,
    camera::CameraPlugin,
    test::TestPlugin,
};

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>() // EXTERN: // EXTERN: Add NavMesh support
            .add(CorePlugin)
            .add_group(NavigationPlugins) // EXTERN: Generation NavMesh
            .add(InputPlugin)
            .add(CameraPlugin)
            .add(ActorsPlugin)
            .add(TestPlugin)
    }
}
