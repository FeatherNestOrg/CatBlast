use bevy::prelude::*;

/// Marker component for buttons that can be focused
#[derive(Component)]
pub struct Focusable;

/// Component indicating a button is currently focused
#[derive(Component)]
pub struct Focused;

/// Component indicating a button is in a selected state (e.g., current resolution)
#[derive(Component)]
pub struct Selected;
