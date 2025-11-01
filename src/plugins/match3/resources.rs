use bevy::prelude::*;
use std::alloc::Layout;

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

#[derive(Resource)]
pub struct Board {
    pub width: u32,
    pub height: u32,
    pub grid: Vec<Vec<Option<Entity>>>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![None; height as usize]; width as usize],
        }
    }
}

#[derive(Resource)]
pub struct GemAtlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct SelectionState {
    pub selected_gem: Option<Entity>,
}