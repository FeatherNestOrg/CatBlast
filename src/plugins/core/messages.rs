use crate::plugins::core::resources::Resolution;
use bevy::prelude::*;
use bevy::window::WindowMode;

/// Event triggered when user wants to apply new display settings
#[derive(Message)]
pub struct ApplyDisplaySettingsEvent {
    pub resolution: Resolution,
    pub window_mode: WindowMode,
}
