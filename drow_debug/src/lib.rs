pub mod prelude;
use bevy::app::{
    PluginGroup,
    PluginGroupBuilder,
};
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    quick::WorldInspectorPlugin,
};

pub struct DebugPlugins;
impl PluginGroup for DebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EguiPlugin::default())
            .add(WorldInspectorPlugin::new())
    }
}
