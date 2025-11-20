use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Default)]
pub struct NavigationNeighbors {
    pub up: Option<Entity>,
    pub down: Option<Entity>,
    pub left: Option<Entity>,
    pub right: Option<Entity>,
}

impl NavigationNeighbors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_up(mut self, entity: Option<Entity>) -> Self {
        self.up = entity;
        self
    }

    pub fn with_down(mut self, entity: Option<Entity>) -> Self {
        self.down = entity;
        self
    }

    pub fn with_left(mut self, entity: Option<Entity>) -> Self {
        self.left = entity;
        self
    }

    pub fn with_right(mut self, entity: Option<Entity>) -> Self {
        self.right = entity;
        self
    }
}

/// Resource to track navigation relationships between buttons
#[derive(Resource, Default)]
pub struct NavigationGraph {
    /// Maps button entities to their navigation neighbors
    pub relationships: HashMap<Entity, NavigationNeighbors>,
    /// Current focused button (if any)
    pub focused_button: Option<Entity>,
}

impl NavigationGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a button with its navigation neighbors
    pub fn register_button(&mut self, entity: Entity, neighbors: NavigationNeighbors) {
        self.relationships.insert(entity, neighbors);
    }

    /// Get the neighbor in a specific direction
    pub fn get_neighbor(&self, entity: Entity, direction: Direction) -> Option<Entity> {
        self.relationships
            .get(&entity)
            .and_then(|neighbors| match direction {
                Direction::Up => neighbors.up,
                Direction::Down => neighbors.down,
                Direction::Left => neighbors.left,
                Direction::Right => neighbors.right,
            })
    }

    /// Set the currently focused button
    pub fn set_focus(&mut self, entity: Entity) {
        self.focused_button = Some(entity);
    }

    /// Clear the focused button
    pub fn clear_focus(&mut self) {
        self.focused_button = None;
    }

    /// Get the currently focused button
    pub fn get_focused(&self) -> Option<Entity> {
        self.focused_button
    }

    /// Remove a button from the graph
    pub fn remove_button(&mut self, entity: Entity) {
        self.relationships.remove(&entity);
        if self.focused_button == Some(entity) {
            self.focused_button = None;
        }
    }

    /// Clear all relationships
    pub fn clear(&mut self) {
        self.relationships.clear();
        self.focused_button = None;
    }
}
