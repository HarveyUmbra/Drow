use bevy::prelude::*;
use drow_core::prelude::*;

pub fn on_play(_trigger: On<Pointer<Click>>, mut res: ResMut<NextState<AppState>>) {
    res.set(AppState::Game);
}

pub fn on_quit(_trigger: On<Pointer<Click>>, mut message_writer: MessageWriter<AppExit>) {
    message_writer.write(AppExit::Success);
}

pub fn on_menu(_trigger: On<Pointer<Click>>, mut res: ResMut<NextState<AppState>>) {
    res.set(AppState::Menu);
}

pub fn on_back(_trigger: On<Pointer<Click>>, mut res: ResMut<NextState<GameState>>) {
    res.set(GameState::Run);
}
