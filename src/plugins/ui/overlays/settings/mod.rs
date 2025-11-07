mod components;
mod systems;

use crate::plugins::ui::overlays::settings::systems::{
    cleanup_settings_ui, settings_button_interaction_system, setup_settings_ui,
    update_window_mode_label_system,
};
use crate::plugins::ui::overlays::setup_overlay_background;
use crate::state::{GameState, OverlayState};
use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(OverlayState::Settings), setup_settings_ui)
            .add_systems(
                Update,
                (
                    settings_button_interaction_system,
                    update_window_mode_label_system,
                )
                    .run_if(in_state(OverlayState::Settings)),
            )
            .add_systems(OnExit(OverlayState::Settings), cleanup_settings_ui);
    }
}
