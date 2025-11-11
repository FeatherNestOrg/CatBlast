mod actions;
pub use actions::GlobalAction;
mod components;
pub use components::GlobalInputController;
pub mod messages;
pub mod resources;
pub mod systems;

use crate::plugins::core::messages::ApplyDisplaySettingsMessage;
use crate::plugins::core::systems::{
    apply_display_settings_system, setup_display_settings, setup_global_input,
    setup_menu_navigation_input,
};
use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            // .init_resource::<DisplaySettings>()
            .add_message::<ApplyDisplaySettingsMessage>()
            .add_systems(
                Startup,
                (
                    setup_display_settings,
                    setup_global_input,
                    setup_menu_navigation_input,
                ),
            )
            .add_systems(Update, apply_display_settings_system);
    }
}
