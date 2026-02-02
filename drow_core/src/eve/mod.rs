pub mod request;
use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Event, Default)]
pub struct SpawnActorEvent {
    pub position: Position,
    pub rotation: Rotation,
}
