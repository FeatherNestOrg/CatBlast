use bevy::prelude::*;

#[derive(Component)]
pub struct Gem;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition{
    pub x: u32,
    pub y: u32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GemType {
    Paw,
    Yarn,
    Fish,
}

#[derive(Component)]
pub struct Selected;