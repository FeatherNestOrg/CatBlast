mod components;
mod systems;

use crate::plugins::ui::settings::systems::{
    cleanup_settings_ui, settings_button_interaction_system, setup_settings_ui,
};
use crate::state::GameState;
use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), setup_settings_ui)
            .add_systems(
                Update,
                settings_button_interaction_system.run_if(in_state(GameState::Settings)),
            )
            .add_systems(OnExit(GameState::Settings), cleanup_settings_ui);
    }
}
