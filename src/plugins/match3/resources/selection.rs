use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SelectionState {
    pub selected_gem: Option<Entity>,
}

