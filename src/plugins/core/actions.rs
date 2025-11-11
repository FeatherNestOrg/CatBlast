use bevy::prelude::Reflect;
use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GlobalAction {
    ToggleMenu,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum MenuNavigationAction {
    Up,
    Down,
    Left,
    Right,
    Select,
}
