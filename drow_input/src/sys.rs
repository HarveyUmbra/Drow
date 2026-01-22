use super::eve::*;
use bevy::prelude::*;

// Übersetzt den User Input in ein Move Event das eine richtung wiedergibt
pub fn controler_wasd(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>, //
) {
    let mut direction = Vec3::ZERO;
    direction.z += keyboard.pressed(KeyCode::KeyW) as i32 as f32;
    direction.z -= keyboard.pressed(KeyCode::KeyS) as i32 as f32;
    direction.x += keyboard.pressed(KeyCode::KeyA) as i32 as f32;
    direction.x -= keyboard.pressed(KeyCode::KeyD) as i32 as f32;
    if let Ok(dir) = Dir3::new(direction) {
        commands.trigger(MoveInput(dir));
    }
}

// Übersetzt den User Input in ein Rotate Event das eine richtung wiedergibt
pub fn controler_eq(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>, //
) {
    let mut direction = 0.0;
    direction += keyboard.just_pressed(KeyCode::KeyQ) as i32 as f32;
    direction -= keyboard.just_pressed(KeyCode::KeyE) as i32 as f32;

    if direction != 0.0 {
        commands.trigger(RotateInput(direction));
    }
}
