use bevy::prelude::*;

#[derive(Event)]
pub struct MoveEvent(pub Dir3);
#[derive(Event)]
pub struct RotateEvent(pub f32);
