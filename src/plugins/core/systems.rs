use crate::plugins::core::components::{GlobalInputController, MenuNavigationInputController};
use crate::plugins::core::messages::ApplyDisplaySettingsMessage;
use crate::plugins::core::resources::{DisplaySettings, MonitorInfo, Resolution};
use bevy::prelude::*;
use bevy::window::{Monitor, PrimaryWindow};
use leafwing_input_manager::plugin::InputManagerSystem;

use crate::plugins::core::GlobalAction;
use crate::plugins::core::actions::MenuNavigationAction;
use leafwing_input_manager::prelude::*;

pub fn setup_display_settings(mut commands: Commands, monitors: Query<(Entity, &Monitor)>) {
    let mut settings = DisplaySettings::default();

    let mut info: Vec<MonitorInfo> = Vec::new();
    for (entity, monitor) in monitors.iter() {
        debug!("Found monitor: {:?}", monitor);
        let mut resolutions: Vec<Resolution> = Vec::new();
        for modes in monitor.video_modes.iter() {
            resolutions.push(Resolution {
                width: modes.physical_size.x,
                height: modes.physical_size.y,
            });
        }
        info.push(MonitorInfo {
            entity,
            name: monitor.name.clone(),
            resolutions,
        });
    }
    settings.monitor_infos = info;
    settings.current_resolution = settings
        .monitor_infos
        .first()
        .and_then(|m| m.resolutions.first())
        .cloned()
        .unwrap_or(Resolution {
            width: 800,
            height: 600,
        });
    commands.insert_resource(settings)
}

/// System that listens for ApplyDisplaySettingsEvent and applies window/resolution changes
pub fn apply_display_settings_system(
    mut display_settings: ResMut<DisplaySettings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut reader: MessageReader<ApplyDisplaySettingsMessage>,
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
            window.resolution.set(
                event.resolution.width as f32,
                event.resolution.height as f32,
            );
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

pub fn setup_global_input(mut commands: Commands) {
    let input_map = InputMap::new([(GlobalAction::ToggleMenu, KeyCode::Escape)]);
    commands.spawn(input_map).insert(GlobalInputController);
}

pub fn setup_menu_navigation_input(mut commands: Commands) {
    let input_map = InputMap::new([
        (MenuNavigationAction::Up, KeyCode::ArrowUp),
        (MenuNavigationAction::Down, KeyCode::ArrowDown),
        (MenuNavigationAction::Left, KeyCode::ArrowLeft),
        (MenuNavigationAction::Right, KeyCode::ArrowRight),
        (MenuNavigationAction::Select, KeyCode::Enter),
    ]);
    commands
        .spawn(input_map)
        .insert(MenuNavigationInputController);
}
