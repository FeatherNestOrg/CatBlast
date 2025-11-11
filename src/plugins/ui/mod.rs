use crate::plugins::ui::main_menu::MainMenuPlugin;
use crate::plugins::ui::overlays::{OverlayMessage, OverlayPlugin};
use crate::plugins::ui::resources::MenuStack;
use crate::plugins::ui::systems::menu_stack_control_system;
use crate::state::OverlayState;
use bevy::prelude::*;

pub mod main_menu;
pub mod overlays;
mod resources;
mod styles;
pub mod systems;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<OverlayState>()
            .init_resource::<MenuStack>()
            .add_plugins(OverlayPlugin)
            .add_plugins(MainMenuPlugin)
            .add_systems(Update, menu_stack_control_system);
    }
}
