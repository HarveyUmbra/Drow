use super::eve::*;
use avian3d::prelude::*;
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

pub fn controler_click(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    spaciel: SpatialQuery,
    windows: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let pressed = mouse.get_just_pressed().next();
    let filter = match pressed {
        Some(MouseButton::Left) => SpatialQueryFilter::from_mask(32), // Todo Defination of Layers
        Some(MouseButton::Right) => SpatialQueryFilter::from_mask(43), // Todo Defination of Layers
        _ => return,
    };

    if let Some(cursor_pos) = windows.cursor_position() {
        if let Ok(ray) = camera.0.viewport_to_world(camera.1, cursor_pos) {
            if let Some(hit) = spaciel.cast_ray(
                ray.origin, //
                ray.direction,
                1000.0,
                true,
                &filter,
            ) {
                match pressed {
                    Some(MouseButton::Left) => {
                        commands.trigger(ClickLeftInput(hit));
                        info!("Click Entity {} whit Left", hit.entity)
                    }
                    Some(MouseButton::Right) => {
                        commands.trigger(ClickRightInput(hit));
                        info!("Click Entity {} whit Rigth", hit.entity)
                    }
                    _ => {
                        info!("Click False Entity")
                    }
                }
            } else {
                info!("Dont Click Enitty");
            }
        }
    }
}
