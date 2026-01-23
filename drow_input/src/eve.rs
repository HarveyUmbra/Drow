use avian3d::prelude::RayHitData;
use bevy::prelude::*;

#[derive(Event)]
pub struct MoveInput(pub Dir3);
#[derive(Event)]
pub struct RotateInput(pub f32);
#[derive(Event)]
pub struct ClickLeftInput(pub RayHitData);
#[derive(Event)]
pub struct ClickRightInput(pub RayHitData);
