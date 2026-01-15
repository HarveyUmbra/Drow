mod com;
pub mod prelude;
mod sys;
use self::sys::*;
use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
};
use vleue_navigator::prelude::*;

pub struct NavigationPlugin;
impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (compute_path, display_path));
    }
}

pub struct NavigationPlugins;
impl PluginGroup for NavigationPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(VleueNavigatorPlugin)
    }
}
