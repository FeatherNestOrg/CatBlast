mod state;
mod plugins;

use bevy::prelude::*;
use crate::state::GameState;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .run();
}