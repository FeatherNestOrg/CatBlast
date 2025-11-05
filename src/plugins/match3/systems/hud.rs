use bevy::prelude::*;
use crate::plugins::core::resources::DisplaySettings;
use crate::plugins::match3::resources::{
    HudLayout
};
use crate::plugins::match3::components::hud::{TopScorePanel, LeftAllyPanel, RightEnemyPanel, BottomPowerUpPanel, HudElement};


pub fn setup_hud_layout(
    mut commands: Commands,
    display_settings: Res<DisplaySettings>,
) {
    let width = display_settings.current_resolution.width as f32;
    let height = display_settings.current_resolution.height as f32;

    let hud_layout = HudLayout::from_resolution(width, height);

    info!("Initializing HUD layout for resolution: {}x{}", width, height);
    debug!("Board available area: {}x{}", hud_layout.board_available_width, hud_layout.board_available_height);

    commands.insert_resource(hud_layout);
}

pub fn setup_hud_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hud_layout: Res<HudLayout>,
) {
    let font = TextFont {
        font: asset_server.load("fonts/ZCOOLKuaiLe-Regular.ttf"),
        font_size: 24.0,
        ..default()
    };
    let top_percent = (hud_layout.top_panel_height / hud_layout.window_height) * 100.0;
    let left_percent = (hud_layout.left_panel_width / hud_layout.window_width) * 100.0;
    let right_percent = (hud_layout.right_panel_width / hud_layout.window_width) * 100.0;
    let bottom_percent = (hud_layout.bottom_panel_height / hud_layout.window_height) * 100.0;
    let middle_height_percent = 100.0 - top_percent - bottom_percent;

    commands.spawn((
        Node {
            width: percent(100.0),
            height: percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
    ))
        .with_children(|root| {
            // ===== 顶部计分板面板 =====
            root.spawn((
                Node {
                    width: percent(100.0),
                    height: percent(top_percent),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    flex_direction: FlexDirection::Row,
                    padding: UiRect::all(px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
                TopScorePanel,
                HudElement,
            ))
                .with_children(|panel| {
                    panel.spawn((
                        Text::new("得分: 0"),
                        font.clone(),
                        TextColor(Color::srgb(0.2, 0.8, 0.2)),
                    ));

                    panel.spawn((
                        Text::new("连击: 0x"),
                        font.clone(),
                        TextColor(Color::srgb(1.0, 0.8, 0.0)),
                    ));

                    panel.spawn((
                        Text::new("回合: 1"),
                        font.clone(),
                        TextColor(Color::srgb(0.5, 0.7, 1.0)),
                    ));
                });
            root.spawn((
                Node {
                    width: percent(left_percent),
                    height: percent(middle_height_percent),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(top_percent),
                    left: Val::Px(0.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: px(10.0),
                    column_gap: px(10.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.05, 0.15, 0.05)),
                LeftAllyPanel,
                HudElement,
            ))
                .with_children(|panel| {
                    // 角色头像
                    panel.spawn((
                        Node {
                            width: px(80.0),
                            height: px(80.0),
                            border: UiRect::all(px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.4, 0.2)),
                    ));

                    panel.spawn((
                        Text::new("友方"),
                        font.clone(),
                        TextColor(Color::srgb(0.4, 0.9, 0.4)),
                    ));

                    // HP 条
                    panel.spawn((
                        Node {
                            width: px(100.0),
                            height: px(20.0),
                            border: UiRect::all(px(1.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                        BorderColor {
                            top: Color::srgb(0.4, 0.8, 0.4),
                            right: Color::srgb(0.4, 0.8, 0.4),
                            bottom: Color::srgb(0.4, 0.8, 0.4),
                            left: Color::srgb(0.4, 0.8, 0.4),
                        },
                    ))
                        .with_children(|hp| {
                            hp.spawn((
                                Node {
                                    width: percent(100.0),
                                    height: percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                            ));
                        });

                    panel.spawn((
                        Text::new("HP: 100/100"),
                        font.clone(),
                        TextColor(Color::srgb(0.8, 0.2, 0.2)),
                    ));
                });
            // ===== 右侧敌人面板 =====
            root.spawn((
                Node {
                    width: percent(right_percent),
                    height: percent(middle_height_percent),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(top_percent),
                    right: Val::Px(0.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: px(10.0),
                    column_gap: px(10.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.05, 0.05)),
                RightEnemyPanel,
                HudElement,
            ))
                .with_children(|panel| {
                    panel.spawn((
                        Node {
                            width: px(80.0),
                            height: px(80.0),
                            border: UiRect::all(px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.4, 0.2, 0.2)),
                        BorderColor {
                            top: Color::srgb(0.8, 0.4, 0.4),
                            right: Color::srgb(0.8, 0.4, 0.4),
                            bottom: Color::srgb(0.8, 0.4, 0.4),
                            left: Color::srgb(0.8, 0.4, 0.4),
                        },
                    ));

                    panel.spawn((
                        Text::new("敌人"),
                        font.clone(),
                        TextColor(Color::srgb(0.9, 0.4, 0.4)),
                    ));

                    panel.spawn((
                        Node {
                            width: px(100.0),
                            height: px(20.0),
                            border: UiRect::all(px(1.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                        BorderColor {
                            top: Color::srgb(0.8, 0.4, 0.4),
                            right: Color::srgb(0.8, 0.4, 0.4),
                            bottom: Color::srgb(0.8, 0.4, 0.4),
                            left: Color::srgb(0.8, 0.4, 0.4),
                        },
                    ))
                        .with_children(|hp| {
                            hp.spawn((
                                Node {
                                    width: percent(75.0),
                                    height: percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                            ));
                        });

                    panel.spawn((
                        Text::new("HP: 75/100"),
                        font.clone(),
                        TextColor(Color::srgb(0.8, 0.2, 0.2)),
                    ));
                });
            // ===== 底部道具栏面板 =====
            root.spawn((
                Node {
                    width: percent(100.0),
                    height: percent(bottom_percent),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    left: Val::Px(0.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    padding: UiRect::all(px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
                BottomPowerUpPanel,
                HudElement,
            ))
                .with_children(|panel| {
                    for i in 0..4 {
                        panel
                            .spawn((
                                Node {
                                    width: px(70.0),
                                    height: px(70.0),
                                    border: UiRect::all(px(2.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
                                BorderColor {
                                    top: Color::srgb(0.5, 0.5, 0.6),
                                    right: Color::srgb(0.5, 0.5, 0.6),
                                    bottom: Color::srgb(0.5, 0.5, 0.6),
                                    left: Color::srgb(0.5, 0.5, 0.6),
                                },
                            ))
                            .with_children(|slot| {
                                slot.spawn((
                                    Text::new(format!("P{}", i + 1)),
                                    font.clone(),
                                    TextColor(Color::srgb(0.7, 0.7, 0.8)),
                                ));
                            });
                    }
                });
        });
}

pub fn update_hud_on_resolution_change(
    display_settings: Res<DisplaySettings>,
    mut hud_layout: ResMut<HudLayout>,
) {
    let current_width = display_settings.current_resolution.width as f32;
    let current_height = display_settings.current_resolution.height as f32;

    if hud_layout.window_width == current_width && hud_layout.window_height == current_height {
        return;
    }

    info!(
        "Resolution changed: {}x{} -> {}x{}",
        hud_layout.window_width, hud_layout.window_height, current_width, current_height
    );

    *hud_layout = HudLayout::from_resolution(current_width, current_height);
}

pub fn cleanup_hud_ui(
    mut commands: Commands,
    query: Query<Entity, With<HudElement>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}