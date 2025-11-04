use bevy::prelude::*;

#[derive(Component)]
pub struct OnSettingsScreen;

#[derive(Component)]
pub enum SettingsButtonAction {
    SelectResolution(usize), // Index into available_resolutions
    ToggleWindowMode,
    Back,
}
