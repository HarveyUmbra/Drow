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

#[derive(Event)] // Todo EntityObserver
pub struct SelectActorsRequest {
    pub entities: Vec<Entity>,
}
impl SelectActorsRequest {
    pub fn new(entities: Vec<Entity>) -> SelectActorsRequest {
        return SelectActorsRequest { entities: entities };
    }
}

#[derive(Event)] // Todo EntityObserver
pub struct DeselectActorsRequest {
    pub exceptions_entities: Vec<Entity>, //
}

impl DeselectActorsRequest {
    pub fn new(entities: Vec<Entity>) -> DeselectActorsRequest {
        return DeselectActorsRequest {
            exceptions_entities: entities,
        };
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
