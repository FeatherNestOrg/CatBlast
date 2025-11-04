mod plugins;
mod state;

use crate::state::GameState;
use bevy::prelude::*;
use plugins::core::CorePlugin;
use plugins::match3::Match3Plugin;
use plugins::ui::main_menu::MainMenuPlugin;
use plugins::ui::settings::SettingsPlugin;
use tracing_subscriber::EnvFilter;

fn main() {
    #[cfg(debug_assertions)]
    {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("CatBlast=debug"));

        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    }
    #[cfg(not(debug_assertions))]
    {
        let file = std::fs::File::create("game.log").expect("Failed to create log file");
        let env_filter = EnvFilter::new("catblast=info");

        tracing_subscriber::fmt()
            .with_writer(file)
            .with_env_filter(env_filter)
            .init();
    }

    App::new()
        .add_plugins((
            DefaultPlugins,
            CorePlugin,
            Match3Plugin,
            MainMenuPlugin,
            SettingsPlugin,
        ))
        .init_state::<GameState>()
        .insert_state(GameState::MainMenu)
        .run();
}
