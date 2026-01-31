use super::com::*;
use bevy::prelude::*;
use drow_core::prelude::*;

pub fn spawn_camera_setup(mut commands: Commands) {
    commands
        .spawn((
            Transform::from_translation(Vec3::ZERO), //
            Player,
            DespawnOnExit(AppState::Game),
        ))
        .with_child((
            Camera3d::default(),
            Transform::from_xyz(0.0, 10.0, -10.0) //
                .looking_at(Vec3::ZERO, Vec3::Y),
        ));
}

pub fn movement(
    event: On<MoveRequest>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let direction = event.direction;

    for mut transform in query.iter_mut() {
        let world_direction = transform.rotation * *direction;
        transform.translation += world_direction * 5.0 * time.delta_secs();
    }
}

pub fn rotate(event: On<RotateRequest>, mut query: Query<&mut Transform, With<Player>>) {
    let direction = event.direction;

    for mut transform in query.iter_mut() {
        transform.rotate_axis(Dir3::Y, direction * 90.0f32.to_radians());
    }
}
