mod navmesh;
pub mod prelude;

use avian_rerecast::{
    AvianBackendPlugin,
    prelude::*,
};
use bevy::{
    app::{
        App,
        Plugin,
        PluginGroup,
        PluginGroupBuilder,
    },
    prelude::*,
};
use bevy_rerecast::{
    debug::{
        DetailNavmeshGizmo,
        NavmeshDebugPlugin,
    },
    generator,
    prelude::*,
};

pub struct NavigationPlugin;
impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_navmesh);
    }
}

pub struct NavigationPlugins;
impl PluginGroup for NavigationPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AvianBackendPlugin::default())
            .add(NavmeshDebugPlugin::default())
            //.add(NavigationPlugin)
            .add_group(NavmeshPlugins::default())
    }
}

#[derive(Resource)]
#[allow(dead_code)]
struct NavmeshHandle(Handle<Navmesh>);

fn generate_navmesh(mut generator: NavmeshGenerator, mut commands: Commands) {
    let settings = NavmeshSettings::default();
    let navmesh = generator.generate(settings);
    commands.spawn(DetailNavmeshGizmo::new(&navmesh));
    commands.insert_resource(NavmeshHandle(navmesh));
}
