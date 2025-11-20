use crate::plugins::ui::main_menu::MainMenuPlugin;
use crate::plugins::ui::navigation::NavigationGraph;
use crate::plugins::ui::overlays::OverlayPlugin;
use crate::plugins::ui::resources::MenuStack;
use crate::plugins::ui::systems::menu_stack_control_system;
use crate::plugins::ui::systems::navigation::{
    ButtonStyleResource, cleanup_despawned_buttons, handle_navigation_input,
    universal_button_style_system,
};
use crate::state::OverlayState;
use bevy::input_focus::InputDispatchPlugin;
use bevy::input_focus::directional_navigation::DirectionalNavigationPlugin;
use bevy::prelude::*;

pub mod button_builder;
pub mod components;
pub mod main_menu;
pub mod navigation;
pub mod overlays;
mod resources;
mod styles;
pub mod systems;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<OverlayState>()
            .init_resource::<MenuStack>()
            .init_resource::<NavigationGraph>()
            .init_resource::<ButtonStyleResource>()
            .add_plugins(InputDispatchPlugin)
            .add_plugins(DirectionalNavigationPlugin)
            .add_plugins(OverlayPlugin)
            .add_plugins(MainMenuPlugin)
            .add_systems(Update, menu_stack_control_system)
            .add_systems(
                Update,
                (
                    handle_navigation_input,
                    universal_button_style_system,
                    cleanup_despawned_buttons,
                ),
            );
    }
}
