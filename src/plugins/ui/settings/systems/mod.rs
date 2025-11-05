use crate::plugins::core::messages::ApplyDisplaySettingsEvent;
use crate::plugins::core::resources::DisplaySettings;
use crate::plugins::ui::settings::components::{OnSettingsScreen, SettingsButtonAction};
use crate::state::GameState;
use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const SELECTED_BUTTON: Color = Color::srgb(0.2, 0.5, 0.2);

fn window_mode_to_chinese(mode: WindowMode) -> &'static str {
    match mode {
        WindowMode::Windowed => "窗口模式",
        WindowMode::BorderlessFullscreen(_) => "无边框全屏",
        WindowMode::Fullscreen(_, _) => "全屏",
    }
}

pub fn setup_settings_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    display_settings: Res<DisplaySettings>,
) {
    let font = TextFont {
        font: asset_server.load("fonts/ZCOOLKuaiLe-Regular.ttf"),
        ..default()
    };

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
            for (index, resolution) in display_settings.monitor_resolutions.iter().enumerate() {
                let is_selected = *resolution == display_settings.current_resolution;
                let button_color = if is_selected {
                    SELECTED_BUTTON
                } else {
                    NORMAL_BUTTON
                };

                parent
                    .spawn((
                        Button,
                        Node {
                            width: px(250.0),
                            height: px(50.0),
                            margin: UiRect::all(px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(button_color),
                        SettingsButtonAction::SelectResolution(index),
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new(resolution.to_string()),
                            font.clone(),
                            TextLayout::new_with_justify(Justify::Center),
                        ));
                    });
            }

            // Spacing
            parent.spawn(Node {
                height: px(20.0),
                ..default()
            });

            // Window mode section
            parent.spawn((
                Text::new(format!("窗口模式: {}", window_mode_to_chinese(display_settings.window_mode))),
                font.clone(),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: px(250.0),
                        height: px(50.0),
                        margin: UiRect::all(px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    SettingsButtonAction::ToggleWindowMode,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("切换窗口模式"),
                        font.clone(),
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                });

            // Spacing
            parent.spawn(Node {
                height: px(20.0),
                ..default()
            });

            // Back button
            parent
                .spawn((
                    Button,
                    Node {
                        width: px(250.0),
                        height: px(50.0),
                        margin: UiRect::all(px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    SettingsButtonAction::Back,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("返回"),
                        font.clone(),
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                });
        });
}

pub fn settings_button_interaction_system(
    mut q_interaction: Query<(&Interaction, &mut BackgroundColor, &SettingsButtonAction)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut apply_settings_writer: MessageWriter<ApplyDisplaySettingsEvent>,
    display_settings: Res<DisplaySettings>,
) {
    for (interaction, mut color, action) in &mut q_interaction {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match action {
                    SettingsButtonAction::SelectResolution(index) => {
                        if let Some(&resolution) = display_settings.monitor_resolutions.get(*index)
                        {
                            info!("Selected resolution: {}", resolution);
                            apply_settings_writer.write(ApplyDisplaySettingsEvent {
                                resolution,
                                window_mode: display_settings.window_mode,
                            });
                        }
                    }
                    SettingsButtonAction::ToggleWindowMode => {
                        let new_mode = match display_settings.window_mode {
                            WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                            WindowMode::BorderlessFullscreen(_) => WindowMode::Windowed,
                            WindowMode::Fullscreen(_, _) => WindowMode::Windowed,
                        };
                        info!("Toggling window mode to: {:?}", new_mode);
                        apply_settings_writer.write(ApplyDisplaySettingsEvent {
                            resolution: display_settings.current_resolution,
                            window_mode: new_mode,
                        });
                    }
                    SettingsButtonAction::Back => {
                        info!("Back button clicked, returning to main menu");
                        next_state.set(GameState::MainMenu);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                // Check if this resolution button should be highlighted as selected
                if let SettingsButtonAction::SelectResolution(index) = action {
                    if let Some(&resolution) = display_settings.monitor_resolutions.get(*index) {
                        if resolution == display_settings.current_resolution {
                            *color = SELECTED_BUTTON.into();
                            continue;
                        }
                    }
                }
                *color = NORMAL_BUTTON.into();
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
