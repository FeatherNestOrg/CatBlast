pub mod character;
pub mod gem;
pub mod animation;

use bevy::prelude::*;
use bevy::prelude::TimerMode::Once;

#[derive(Component)]
pub struct OnMatch3Scene;


#[derive(Component)]
pub struct Selected {
    pub timer: Timer,
}

impl Selected {
    pub fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, Once),
        }
    }
}
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}
