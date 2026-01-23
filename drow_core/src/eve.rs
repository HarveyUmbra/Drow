use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct MoveRequest {
    pub direction: Dir3,
}
impl MoveRequest {
    pub fn new(direction: Dir3) -> MoveRequest {
        return MoveRequest {
            direction: direction,
        };
    }
}

#[derive(Event)]
pub struct RotateRequest {
    pub direction: f32,
}
impl RotateRequest {
    pub fn new(direction: f32) -> RotateRequest {
        return RotateRequest {
            direction: direction,
        };
    }
}

#[derive(Event)]
pub struct SelectActorRequest {
    pub entity: Entity,
}
impl SelectActorRequest {
    pub fn new(entity: Entity) -> SelectActorRequest {
        return SelectActorRequest { entity: entity };
    }
}

#[derive(Event)]
pub struct ChangeTargetRequest {
    pub entity: Entity,
    pub target: Vec3,
}

impl ChangeTargetRequest {
    pub fn new(entity: Entity, target: Vec3) -> ChangeTargetRequest {
        return ChangeTargetRequest {
            entity: entity,
            target: target,
        };
    }
}
