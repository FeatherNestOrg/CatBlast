use crate::plugins::match3::system::{gem_input_system, gem_selection_system};
use crate::plugins::match3::components::{Gem, GemType, GridPosition};
use crate::plugins::match3::resources::{GemAtlas, Match3Config, SelectionState};
use crate::state::GameState;
use bevy::prelude::*;
use resources::Board;
use crate::plugins::match3::message::{GemClickedEvent, RequestSwapEvent};

mod components;
mod resources;
mod message;
mod system;

pub struct Match3Plugin;

impl Plugin for Match3Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<GemClickedEvent>()
            .add_message::<RequestSwapEvent>()
            .init_resource::<Match3Config>()
            .init_resource::<SelectionState>()
            .add_systems(
                OnEnter(GameState::Match3),
                (setup_gem_atlas, setup_match3_scene).chain(),
            )
            .add_systems(
                Update,
                (gem_input_system,
                 gem_selection_system,
                 match_detection_system)
                    .run_if(in_state(GameState::Match3)),
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

fn setup_match3_scene(mut commands: Commands, gem_atlas: Res<GemAtlas>, config: Res<Match3Config>) {
    println!("Entering Match3 Scene! Let's set up the board.");
    commands.spawn(Camera2d);

    let board = Board::new(config.board_width, config.board_height);

    for x in 0..board.width {
        for y in 0..board.height {
            let gem_type = GemType::Ice; // 暂时用一个固定的类型举例

            // 计算正确的图块索引 (第二行索引 = 类型索引 + 8)
            let atlas_index = gem_type as usize + 8;

            commands.spawn((
                Sprite::from_atlas_image(
                    gem_atlas.image.clone(),
                    TextureAtlas {
                        layout: gem_atlas.layout.clone(),
                        index: atlas_index,
                    },
                ),
                Transform::from_xyz(
                    x as f32 * config.gem_size - (board.width as f32 * config.gem_size) / 2.0 + config.gem_size / 2.0,
                    y as f32 * config.gem_size - (board.height as f32 * config.gem_size) / 2.0 + config.gem_size / 2.0,
                    0.0,
                ),
                Gem,
                GridPosition { x, y },
                gem_type,
            ));
        }
    }
    commands.insert_resource(board);
}

fn match_detection_system() {
    /* ... */
}

fn cleanup_match3_scene(mut commands: Commands, query: Query<Entity, With<Gem>>) {
    println!("Cleaning up Match3 Scene.");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
