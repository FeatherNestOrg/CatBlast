use crate::plugins::core::events::ApplyDisplaySettingsEvent;
use crate::plugins::core::resources::{DisplaySettings, Resolution};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// System that runs at startup to detect and store available display configurations
pub fn setup_display_settings(
    mut commands: Commands,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    let mut settings = DisplaySettings::default();

    // Get current window resolution if available
    if let Ok(window) = primary_window.single() {
        settings.current_resolution = Resolution::new(
            window.resolution.width() as u32,
            window.resolution.height() as u32,
        );
        settings.window_mode = window.mode;
    }

    commands.insert_resource(settings);
    info!("Display settings initialized");
}

/// System that listens for ApplyDisplaySettingsEvent and applies window/resolution changes
pub fn apply_display_settings_system(
    mut display_settings: ResMut<DisplaySettings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut reader: MessageReader<ApplyDisplaySettingsEvent>,
) {
    for event in reader.read() {
        info!(
            "Applying display settings: {} in {:?} mode",
            event.resolution, event.window_mode
        );

        // Update the display settings resource
        display_settings.current_resolution = event.resolution;
        display_settings.window_mode = event.window_mode;

        // Apply to the primary window
        if let Ok(mut window) = primary_window.single_mut() {
            window
                .resolution
                .set(event.resolution.width as f32, event.resolution.height as f32);
            window.mode = event.window_mode;

            info!(
                "Display settings applied successfully: {} in {:?} mode",
                event.resolution, event.window_mode
            );
        } else {
            warn!("Could not find primary window to apply display settings");
        }
    }
}
