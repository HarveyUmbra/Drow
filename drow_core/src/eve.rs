use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct MoveRequest {
    pub entity: Entity,
    pub direction: Dir3,
}
impl MoveRequest {
    pub fn new(entity: Entity, direction: Dir3) -> MoveRequest {
        return MoveRequest {
            entity: entity,
            direction: direction,
        };
    }
}

#[derive(EntityEvent)]
pub struct RotateRequest {
    pub entity: Entity,
    pub direction: f32,
}
impl RotateRequest {
    pub fn new(entity: Entity, direction: f32) -> RotateRequest {
        return RotateRequest {
            entity: entity,
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
