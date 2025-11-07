use bevy::prelude::*;
use crate::state::OverlayState;

#[derive(Message)]
pub struct OpenOverlay {
    pub overlay: OverlayState,
}