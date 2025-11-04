use bevy::prelude::*;

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub enum MainMenuButtonAction {
    Play,
    Settings,
    Quit,
}
