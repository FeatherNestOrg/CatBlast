use crate::plugins::core::{MenuNavigationAction, MenuNavigationInputController};
use crate::plugins::ui::button_builder::ButtonStyle;
use crate::plugins::ui::components::{Focusable, Focused, Selected};
use crate::plugins::ui::navigation::{Direction, NavigationGraph};
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

/// System to handle directional navigation input and update focus
pub fn handle_navigation_input(
    q_action: Query<&ActionState<MenuNavigationAction>, With<MenuNavigationInputController>>,
    mut nav_graph: ResMut<NavigationGraph>,
    mut commands: Commands,
    q_focused: Query<Entity, With<Focused>>,
    q_focusable: Query<Entity, With<Focusable>>,
) {
    let action_state = q_action.single();
    let Ok(action_state) = action_state else {
        return;
    };

    let direction = if action_state.just_pressed(&MenuNavigationAction::Up) {
        Some(Direction::Up)
    } else if action_state.just_pressed(&MenuNavigationAction::Down) {
        Some(Direction::Down)
    } else if action_state.just_pressed(&MenuNavigationAction::Left) {
        Some(Direction::Left)
    } else if action_state.just_pressed(&MenuNavigationAction::Right) {
        Some(Direction::Right)
    } else {
        return;
    };
    debug!("direction: {:?}", direction);

    if let Some(dir) = direction {
        // Get the current focused button
        let current_focused = q_focused.iter().next();

        if let Some(current) = current_focused {
            // Try to navigate to a neighbor
            if let Some(neighbor) = nav_graph.get_neighbor(current, dir) {
                // Remove focus from current
                commands.entity(current).remove::<Focused>();
                // Add focus to neighbor
                commands.entity(neighbor).insert(Focused);
                nav_graph.set_focus(neighbor);
                info!("Navigated {:?} to button {:?}", dir, neighbor);
            }
        } else {
            // No button is focused, focus the first focusable button
            if let Some(first_button) = q_focusable.iter().next() {
                commands.entity(first_button).insert(Focused);
                nav_graph.set_focus(first_button);
                info!("Focused first button {:?}", first_button);
            }
        }
    }
}

/// Universal button style system that handles all button visual states
/// Priority: Pressed > Hovered > Focused > Selected > Normal
pub fn universal_button_style_system(
    mut q_buttons: Query<
        (
            &mut BackgroundColor,
            &Interaction,
            Option<&Focused>,
            Option<&Selected>,
        ),
        With<Button>,
    >,
    button_style: Res<ButtonStyleResource>,
) {
    for (mut bg_color, interaction, focused, selected) in q_buttons.iter_mut() {
        let new_color = match *interaction {
            Interaction::Pressed => button_style.0.pressed,
            Interaction::Hovered => button_style.0.hovered,
            Interaction::None => {
                // Check for focused or selected states
                if focused.is_some() {
                    button_style.0.focused
                } else if selected.is_some() {
                    button_style.0.selected
                } else {
                    button_style.0.normal
                }
            }
        };

        *bg_color = new_color.into();
    }
}

/// Resource to store the button style configuration
#[derive(Resource, Default)]
pub struct ButtonStyleResource(pub ButtonStyle);

/// System to clean up navigation graph when buttons are despawned
pub fn cleanup_despawned_buttons(
    mut nav_graph: ResMut<NavigationGraph>,
    mut removed: RemovedComponents<Focusable>,
) {
    for entity in removed.read() {
        nav_graph.remove_button(entity);
    }
}
