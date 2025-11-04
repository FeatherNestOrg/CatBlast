pub mod events;
pub mod resources;
pub mod systems;

use crate::plugins::core::events::ApplyDisplaySettingsEvent;
use crate::plugins::core::systems::{apply_display_settings_system, setup_display_settings};
use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ApplyDisplaySettingsEvent>()
            .add_systems(Startup, setup_display_settings)
            .add_systems(Update, apply_display_settings_system);
    }
}
