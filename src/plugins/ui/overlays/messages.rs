use crate::state::OverlayState;
use bevy::prelude::*;

pub enum OverlayAction {
    Push,
    Pop,
}

#[derive(Message)]
pub struct OverlayMessage {
    pub action: OverlayAction,
    pub overlay: OverlayState,
}
