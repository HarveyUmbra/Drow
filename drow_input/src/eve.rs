use bevy::prelude::*;

#[derive(Event)]
pub struct MoveInput(pub Dir3);
#[derive(Event)]
pub struct RotateInput(pub f32);
