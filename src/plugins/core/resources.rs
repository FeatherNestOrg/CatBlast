use bevy::prelude::*;
use bevy::window::WindowMode;
use std::collections::HashMap;

/// Represents a display resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub entity: Entity,
    pub name: Option<String>,
    pub resolutions: Vec<Resolution>,
}

/// Resource storing all display-related settings
#[derive(Resource)]
pub struct DisplaySettings {
    pub monitor_infos: Vec<MonitorInfo>,
    pub current_resolution: Resolution,
    pub window_mode: WindowMode,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            monitor_infos: default(),
            current_resolution: Resolution::new(1280, 720),
            window_mode: WindowMode::Windowed,
        }
    }
}
