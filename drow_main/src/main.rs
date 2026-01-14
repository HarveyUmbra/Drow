use avian3d::{
    PhysicsPlugins,
    prelude::PhysicsDebugPlugin,
};
use bevy::prelude::*;

use drow_debug::prelude::*;
use drow_lib::prelude::*;
use drow_nav::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Bevy Basic Plugins
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin)) // Avian Plugins
        .add_plugins((GamePlugins, DebugPlugins)) // Own Plugins
        .add_plugins(NavigationPlugins)
        .run();
}
