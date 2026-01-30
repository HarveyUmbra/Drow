use bevy::prelude::*;
use drow_core::GameState;

pub fn on_play(_trigger: On<Pointer<Click>>, mut res: ResMut<NextState<GameState>>) {
    res.set(GameState::Game);
}

pub fn on_quit(_trigger: On<Pointer<Click>>, mut message_writer: MessageWriter<AppExit>) {
    message_writer.write(AppExit::Success);
}
