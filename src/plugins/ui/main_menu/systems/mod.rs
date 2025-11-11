use crate::plugins::ui::main_menu::components::{MainMenuButtonAction, OnMainMenuScreen};
use crate::plugins::ui::overlays::{OverlayAction, OverlayMessage};
use crate::state::{GameState, OverlayState};
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, OnMainMenuScreen));
    let font = TextFont {
        font: asset_server.load("fonts/ZCOOLKuaiLe-Regular.ttf"),
        ..default()
    };
    commands
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column, // 纵向排列
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|builder| {
            // 游戏标题
            builder.spawn((
                Text::new("Cat Blast"),
                font.clone(),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
            ));

            // 封装按钮的辅助函数，避免代码重复
            let mut create_button = |text: &str, action: MainMenuButtonAction| {
                builder
                    .spawn((
                        Button,
                        Node {
                            width: px(250.0),
                            height: px(65.0),
                            margin: UiRect::all(px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(NORMAL_BUTTON),
                        action, // 添加我们的标记组件！
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new(text),
                            font.clone(),
                            TextLayout::new_with_justify(Justify::Center),
                        ));
                    });
            };

            // 创建按钮
            create_button("开始游戏", MainMenuButtonAction::Play);
            create_button("设置", MainMenuButtonAction::Settings);
            create_button("退出", MainMenuButtonAction::Quit);
        });
}

pub fn button_interaction_system(
    mut q_interaction: Query<
        (&Interaction, &mut BackgroundColor, &MainMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut ns_game: ResMut<NextState<GameState>>,
    mut mw_overlay: MessageWriter<OverlayMessage>,
    mut app_exit_mw: MessageWriter<AppExit>,
) {
    for (interaction, mut color, action) in &mut q_interaction {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match action {
                    MainMenuButtonAction::Play => {
                        info!("开始游戏按钮被点击, 切换到 Match3 状态。");
                        ns_game.set(GameState::Match3);
                    }
                    MainMenuButtonAction::Settings => {
                        info!("设置按钮被点击, 切换到设置界面");
                        mw_overlay.write(OverlayMessage {
                            action: OverlayAction::Push,
                            overlay: OverlayState::Settings,
                        });
                    }
                    MainMenuButtonAction::Quit => {
                        app_exit_mw.write(AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn cleanup_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<OnMainMenuScreen>>,
) {
    // 查询所有带有 OnMainMenuScreen 标记的实体并销毁它们
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn();
    }
}
