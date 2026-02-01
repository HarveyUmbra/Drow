use avian3d::{
    PhysicsPlugins,
    prelude::PhysicsDebugPlugin,
};
use bevy::prelude::*;

use drow_actor::prelude::*;
use drow_core::prelude::*;
use drow_debug::prelude::*;
use drow_input::prelude::*;
use drow_lib::prelude::*;
use drow_menu::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Bevy Basic Plugins
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin)) // Avian Plugins
        .add_plugins((
            CorePlugin,
            InputPlugin,
            ActorsPlugin,
            GamePlugins,
            MenuPlugin,
            DebugPlugins,
        )) // Own Plugins
        .run();
}
