use crate::plugins::core::{GlobalAction, GlobalInputController};
use crate::plugins::ui::overlays::{
    OverlayAction, OverlayBackgroundMarker, OverlayMessage, cleanup_overlay_background,
    setup_overlay_background,
};
use crate::plugins::ui::resources::MenuStack;
use crate::state::{GameState, OverlayState};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub fn menu_stack_control_system(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    mut overlay_state: ResMut<NextState<OverlayState>>,
    mut menu_stack: ResMut<MenuStack>,
    q_overlay_bg: Query<Entity, With<OverlayBackgroundMarker>>,
    q_action: Query<&ActionState<GlobalAction>, With<GlobalInputController>>,
    mut mr_overlay: MessageReader<OverlayMessage>,
) {
    let action_state = q_action.single();
    if let Ok(action_state) = action_state
        && action_state.just_pressed(&GlobalAction::ToggleMenu)
    {
        if menu_stack.is_empty() {
            // 空栈，根据状态决定打开何种菜单
            let new_state = match game_state.get() {
                GameState::MainMenu => OverlayState::ExitMenu,
                GameState::Match3 => OverlayState::PauseMenu,
            };
            menu_stack.push(new_state);
            overlay_state.set(new_state);
            info!("Opening menu: {:?}", new_state);
            setup_overlay_background(&mut commands);
        } else {
            // 有状态就弹出
            try_pop_menu(
                &mut commands,
                &mut overlay_state,
                &mut menu_stack,
                q_overlay_bg,
            );
        }
    }
    for message in mr_overlay.read() {
        match message.action {
            OverlayAction::Push => {
                if (menu_stack.is_empty()) {
                    setup_overlay_background(&mut commands)
                }
                menu_stack.push(message.overlay);
                overlay_state.set(message.overlay);
            }
            OverlayAction::Pop => {
                try_pop_menu(
                    &mut commands,
                    &mut overlay_state,
                    &mut menu_stack,
                    q_overlay_bg,
                );
            }
        }
    }
}

fn try_pop_menu(
    mut commands: &mut Commands,
    overlay_state: &mut ResMut<NextState<OverlayState>>,
    menu_stack: &mut ResMut<MenuStack>,
    q_overlay_bg: Query<Entity, With<OverlayBackgroundMarker>>,
) {
    if menu_stack.pop().is_some() {
        if let Some(next_state) = menu_stack.peek() {
            overlay_state.set(*next_state);
            info!("Returning to: {:?}", next_state);
        } else {
            overlay_state.set(OverlayState::None);
            info!("Closing all menus");
            cleanup_overlay_background(&mut commands, q_overlay_bg);
        }
    }
}
