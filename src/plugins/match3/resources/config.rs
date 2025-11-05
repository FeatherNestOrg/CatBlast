use bevy::prelude::*;

#[derive(Resource)]
pub struct Match3Config {
    pub gem_size: f32,
    pub board_width: u32,
    pub board_height: u32,
}

impl Default for Match3Config {
    fn default() -> Self {
        Self {
            gem_size: 40.0,
            board_width: 8,
            board_height: 8,
        }
    }
}

