mod com;
mod eve;
pub mod prelude;
mod res;
use self::res::*;
use bevy::prelude::*;

pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<State<AppState>>()
            .insert_state(AppState::default())
            .add_sub_state::<GameState>();
    }
}
