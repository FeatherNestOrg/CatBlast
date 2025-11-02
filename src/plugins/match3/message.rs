use bevy::prelude::*;

#[derive(Message)]
pub struct GemClickedEvent {
    pub gem_entity: Entity,
}

#[derive(Message)]
pub struct RequestSwapEvent {
    pub entity1: Entity,
    pub entity2: Entity,
}

#[derive(Message)]
pub struct SwapCompletedEvent {
    pub entity1: Entity,
    pub entity2: Entity,
}
