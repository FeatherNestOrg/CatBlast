use crate::plugins::match3::components::{GridPosition};
use crate::plugins::match3::components::gem::{Gem, GemType};
use crate::plugins::match3::message::{GemClickedEvent, RequestSwapEvent, SwapCompletedEvent};
use crate::plugins::match3::resources::{GemAtlas, Match3Config, PendingSwap, SelectionState};
use crate::plugins::match3::state::Match3State;
use crate::plugins::match3::systems::animation::{blast_animation_system, blast_particle_system, check_animation_system, falling_animation_system, swap_animation_system};
use crate::plugins::match3::systems::input::{gem_input_system, gem_selection_system};
use crate::plugins::match3::systems::processing::process_board_state_system;
use crate::plugins::match3::systems::swap::swap_system;
use crate::plugins::match3::systems::visual::{
    animate_selection_effect, apply_selection_effect, remove_selection_effect,
};
use crate::state::GameState;
use bevy::prelude::*;
use bevy::render::view::Hdr;
use resources::Board;
use crate::plugins::match3::systems::regenerate::spawn_new_board;

mod components;
mod message;
mod resources;
mod state;
mod systems;

pub struct Match3Plugin;

impl Plugin for Match3Plugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GemClickedEvent>()
            .add_message::<RequestSwapEvent>()
            .add_message::<SwapCompletedEvent>()
            .init_resource::<Match3Config>()
            .init_resource::<SelectionState>()
            .init_resource::<PendingSwap>()
            .add_sub_state::<Match3State>()
            .add_systems(
                OnEnter(GameState::Match3),
                (setup_gem_atlas, setup_match3_scene).chain(),
            )
            .add_systems(
                Update,
                (
                    gem_input_system,
                    gem_selection_system,
                    swap_system.after(gem_selection_system),
                    animate_selection_effect,
                    apply_selection_effect,
                    remove_selection_effect,
                )
                    .run_if(in_state(Match3State::AwaitingInput)),
            )
            .add_systems(
                Update,
                (
                    swap_animation_system,
                    blast_animation_system,
                    blast_particle_system,
                    falling_animation_system,
                    check_animation_system,
                )
                    .run_if(in_state(Match3State::Animating)),
            )
            .add_systems(
                OnEnter(Match3State::ProcessingBoard),
                (process_board_state_system),
            )
            .add_systems(OnExit(GameState::Match3), cleanup_match3_scene);
    }
}

fn setup_gem_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let gem_texture_handle = asset_server.load("sprites/match3_tiles.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::new(40, 40), 8, 2, None, None);

    let layout_handle = texture_atlas_layouts.add(layout);

    commands.insert_resource(GemAtlas {
        layout: layout_handle,
        image: gem_texture_handle,
    })
}

fn setup_match3_scene(mut commands: Commands, gem_atlas: Res<GemAtlas>, config: Res<Match3Config>, mut next_state: ResMut<NextState<Match3State>>) {
    tracing::info!("Entering Match3 Scene! Let's set up the board.");
    commands.spawn((Camera2d, Hdr, components::OnMatch3Scene));

    let board = Board::new(config.board_width, config.board_height);

    spawn_new_board(&mut commands, &board, &gem_atlas, &config);
    commands.insert_resource(board);
    next_state.set(Match3State::Animating);
}

fn cleanup_match3_scene(mut commands: Commands, query: Query<Entity, With<components::OnMatch3Scene>>) {
    tracing::info!("Cleaning up Match3 Scene.");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
