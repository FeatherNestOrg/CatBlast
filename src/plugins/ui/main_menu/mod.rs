mod components;
mod systems;

use crate::plugins::ui::main_menu::systems::{
    button_interaction_system, cleanup_main_menu, setup_main_menu,
};
use crate::plugins::ui::overlays::OverlayMessage;
use crate::state::GameState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                button_interaction_system.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);
    }
}
