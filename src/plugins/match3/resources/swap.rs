use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PendingSwap {
    pub entities: Option<(Entity, Entity)>,
}

