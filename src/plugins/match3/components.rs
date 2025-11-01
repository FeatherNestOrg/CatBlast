use bevy::platform::collections::Equivalent;
use bevy::prelude::*;
use rand::Rng;

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

impl GemType {
    pub fn random() -> Self {
        const VARIANTS: [GemType; 8] = [
            GemType::Ice,
            GemType::Water,
            GemType::Fairy,
            GemType::Poison,
            GemType::Fight,
            GemType::Fire,
            GemType::Grass,
            GemType::Normal,
        ];
        let mut rng = rand::rng();
        let idx = rng.random_range(0..VARIANTS.len());
        VARIANTS[idx]
    }
}

#[derive(Component)]
pub struct Selected;
