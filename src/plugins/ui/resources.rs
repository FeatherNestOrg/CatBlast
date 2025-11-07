use crate::state::OverlayState;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MenuStack {
    stack: Vec<OverlayState>,
}

impl MenuStack {
    pub fn push(&mut self, state: OverlayState) {
        if state != OverlayState::None {
            self.stack.push(state);
            debug!("MenuStack: push {:?}, depth: {}", state, self.stack.len());
        }
    }

    pub fn pop(&mut self) -> Option<OverlayState> {
        let popped = self.stack.pop();
        if let Some(state) = popped {
            debug!("MenuStack: pop {:?}, depth: {}", state, self.stack.len());
        }
        popped
    }

    pub fn peek(&self) -> Option<&OverlayState> {
        self.stack.last()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    pub fn clear(&mut self) {
        debug!("MenuStack: clear entire stack");
        self.stack.clear();
    }
}
