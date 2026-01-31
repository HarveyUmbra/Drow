mod actors;
mod camera;
pub mod prelude;
mod test;

use bevy::app::{
    PluginGroup,
    PluginGroupBuilder,
};
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
            .add_group(NavigationPlugins)
            .add(CameraPlugin)
            .add(ActorsPlugin)
            .add(TestPlugin)
    }
}
