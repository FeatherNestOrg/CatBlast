use bevy::prelude::*;
use bevy::window::WindowMode;

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

/// Resource storing all display-related settings
#[derive(Resource)]
pub struct DisplaySettings {
    pub available_resolutions: Vec<Resolution>,
    pub current_resolution: Resolution,
    pub window_mode: WindowMode,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            available_resolutions: vec![
                Resolution::new(800, 600),
                Resolution::new(1024, 768),
                Resolution::new(1280, 720),
                Resolution::new(1920, 1080),
            ],
            current_resolution: Resolution::new(1280, 720),
            window_mode: WindowMode::Windowed,
        }
    }
}
