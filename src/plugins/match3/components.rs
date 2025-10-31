use bevy::prelude::*;

#[derive(Component)]
pub struct Gem;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum GemType {
    Ice,
    Water,
    Fairy,
    Poison,
    Fight,
    Fire,
    Grass,
    Normal,
}

#[derive(Component)]
pub struct Selected;
