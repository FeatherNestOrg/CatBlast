use crate::plugins::core::messages::ApplyDisplaySettingsMessage;
use crate::plugins::core::resources::{DisplaySettings, Resolution};
use crate::plugins::ui::button_builder::{
    ButtonNavigationBuilder, NavigationLayout, create_button_node, spawn_button,
};
use crate::plugins::ui::components::Selected;
use crate::plugins::ui::navigation::NavigationGraph;
use crate::plugins::ui::overlays::settings::components::{
    OnSettingsScreen, SettingsButtonAction, WindowModeLabel,
};
use crate::plugins::ui::overlays::{OverlayAction, OverlayMessage};
use crate::state::OverlayState;
use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};

fn window_mode_to_chinese(mode: WindowMode) -> &'static str {
    match mode {
        WindowMode::Windowed => "窗口模式",
        WindowMode::BorderlessFullscreen(_) => "无边框全屏",
        WindowMode::Fullscreen(_, _) => "全屏",
    }
}

fn get_resolution_by_flat_index(
    settings: &DisplaySettings,
    mut flat_index: usize,
) -> Option<Resolution> {
    for monitor in &settings.monitor_infos {
        if flat_index < monitor.resolutions.len() {
            return Some(monitor.resolutions[flat_index]);
        } else {
            flat_index -= monitor.resolutions.len();
        }
    }
    None
}

pub fn setup_settings_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    display_settings: Res<DisplaySettings>,
    mut nav_graph: ResMut<NavigationGraph>,
) {
    let font = TextFont {
        font: asset_server.load("fonts/ZCOOLKuaiLe-Regular.ttf"),
        ..default()
    };

    let mut flat_resolutions: Vec<Resolution> = Vec::new();
    for monitor in &display_settings.monitor_infos {
        for &res in &monitor.resolutions {
            flat_resolutions.push(res);
        }
    }

    // Clear previous navigation graph
    nav_graph.clear();

    // Create button builder
    let mut button_builder = ButtonNavigationBuilder::new(NavigationLayout::Vertical);

    // Track which resolution buttons should be marked as selected
    let mut selected_buttons = Vec::new();

    // Root node
    commands
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            OnSettingsScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("设置"),
                font.clone(),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
            ));

            // Spacing
            parent.spawn(Node {
                height: px(20.0),
                ..default()
            });

            // Resolution section
            parent.spawn((
                Text::new("分辨率："),
                font.clone(),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
            ));

            // Resolution buttons
            for (index, resolution) in flat_resolutions.iter().enumerate() {
                let is_selected = *resolution == display_settings.current_resolution;

                let button_entity = parent
                    .spawn((
                        Button,
                        create_button_node(250.0, 50.0, 5.0),
                        BackgroundColor(Color::BLACK), // Will be managed by universal_button_style_system
                        SettingsButtonAction::SelectResolution(index),
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new(resolution.to_string()),
                            font.clone(),
                            TextLayout::new_with_justify(Justify::Center),
                        ));
                    })
                    .id();

                // Track if this button should be marked as selected
                if is_selected {
                    selected_buttons.push(button_entity);
                }

                button_builder.add_button(button_entity);
            }

            // Spacing
            parent.spawn(Node {
                height: px(20.0),
                ..default()
            });

            // Window mode section
            parent.spawn((
                Text::new(format!(
                    "窗口模式: {}",
                    window_mode_to_chinese(display_settings.window_mode)
                )),
                font.clone(),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
                WindowModeLabel,
            ));

            let toggle_button = spawn_button(
                parent,
                "切换窗口模式",
                SettingsButtonAction::ToggleWindowMode,
                &font,
                create_button_node(250.0, 50.0, 10.0),
            );
            button_builder.add_button(toggle_button);

            // Spacing
            parent.spawn(Node {
                height: px(20.0),
                ..default()
            });

            // Back button
            let back_button = spawn_button(
                parent,
                "返回",
                SettingsButtonAction::Back,
                &font,
                create_button_node(250.0, 50.0, 10.0),
            );
            button_builder.add_button(back_button);
        });

    // Mark selected buttons (outside the closure)
    for entity in selected_buttons {
        commands.entity(entity).insert(Selected);
    }

    // Build navigation graph and set initial focus
    button_builder.build(&mut commands, &mut nav_graph, true);
}

pub fn update_window_mode_label_system(
    display_settings: Res<DisplaySettings>,
    mut query: Query<&mut Text, With<WindowModeLabel>>,
) {
    if !display_settings.is_changed() {
        return;
    }

    let new_text = format!(
        "窗口模式: {}",
        window_mode_to_chinese(display_settings.window_mode)
    );

    for mut text in query.iter_mut() {
        *text = Text::new(new_text.clone());
    }
}
pub fn settings_button_interaction_system(
    mut commands: Commands,
    q_interaction: Query<
        (Entity, &Interaction, &SettingsButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    q_all_res_buttons: Query<(Entity, &SettingsButtonAction), With<Button>>,
    mut mw_overlay: MessageWriter<OverlayMessage>,
    mut apply_settings_writer: MessageWriter<ApplyDisplaySettingsMessage>,
    display_settings: Res<DisplaySettings>,
) {
    for (entity, interaction, action) in &q_interaction {
        if *interaction == Interaction::Pressed {
            match action {
                SettingsButtonAction::SelectResolution(index) => {
                    if let Some(resolution) =
                        get_resolution_by_flat_index(&display_settings, *index)
                    {
                        info!("Selected resolution: {}", resolution);

                        // Remove Selected from all resolution buttons
                        for (btn_entity, btn_action) in q_all_res_buttons.iter() {
                            if matches!(btn_action, SettingsButtonAction::SelectResolution(_)) {
                                commands.entity(btn_entity).remove::<Selected>();
                            }
                        }

                        // Add Selected to the pressed button
                        commands.entity(entity).insert(Selected);

                        apply_settings_writer.write(ApplyDisplaySettingsMessage {
                            resolution,
                            window_mode: display_settings.window_mode,
                        });
                    }
                }
                SettingsButtonAction::ToggleWindowMode => {
                    let new_mode = match display_settings.window_mode {
                        WindowMode::Windowed => {
                            WindowMode::BorderlessFullscreen(MonitorSelection::Current)
                        }
                        WindowMode::BorderlessFullscreen(_) => WindowMode::Windowed,
                        WindowMode::Fullscreen(_, _) => WindowMode::Windowed,
                    };
                    info!("Toggling window mode to: {:?}", new_mode);
                    apply_settings_writer.write(ApplyDisplaySettingsMessage {
                        resolution: display_settings.current_resolution,
                        window_mode: new_mode,
                    });
                }
                SettingsButtonAction::Back => {
                    info!("Back button clicked, returning to main menu");
                    mw_overlay.write(OverlayMessage {
                        action: OverlayAction::Pop,
                        overlay: OverlayState::Settings,
                    });
                }
            }
        }
    }
}

pub fn cleanup_settings_ui(
    mut commands: Commands,
    settings_query: Query<Entity, With<OnSettingsScreen>>,
) {
    for entity in settings_query.iter() {
        commands.entity(entity).despawn();
    }
}
