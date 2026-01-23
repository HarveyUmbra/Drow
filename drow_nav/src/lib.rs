mod navigator;
mod navmesh;
pub mod prelude;

use avian3d::prelude::*;
use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
};
use vleue_navigator::prelude::*;

use self::{
    navigator::sys::*,
    navmesh::sys::*,
};

pub struct NavigationPlugin;
impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (compute_path, display_path))
            .add_observer(setup_ground)
            .add_observer(despawn_ground);
    }
}

pub struct NavigationPlugins;
impl PluginGroup for NavigationPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(VleueNavigatorPlugin)
            .add(NavigationPlugin)
            .add(NavmeshUpdaterPlugin::<Collider, Obstacle>::default())
    }
}

#[derive(Debug, Component)]
pub struct Obstacle;
