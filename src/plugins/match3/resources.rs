use bevy::prelude::*;
use std::alloc::Layout;

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
