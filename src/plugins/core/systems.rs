use std::collections::BTreeSet;
use crate::plugins::core::resources::{DisplaySettings, Resolution};
use bevy::prelude::*;
use bevy::window::{Monitor, PrimaryWindow};
use crate::plugins::core::messages::ApplyDisplaySettingsEvent;


pub fn setup_display_settings(
    mut commands: Commands,
    monitors: Query<(Entity, &Monitor)>,
) {
    let mut settings = DisplaySettings::default();

    for (entity, monitor) in monitors.iter() {
        let mut set: BTreeSet<(u32, u32)> = BTreeSet::new();
       if let Some(vm) = monitor.video_modes.
    }
    settings.current_resolution = settings.monitor_resolutions
    commands.insert_resource(settings);
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
