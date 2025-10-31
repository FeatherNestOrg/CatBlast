mod plugins;
mod state;

use crate::state::GameState;
use bevy::prelude::*;
use plugins::match3::Match3Plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Match3Plugin)
        .init_state::<GameState>()
        .insert_state(GameState::Match3)
        .run();
}
