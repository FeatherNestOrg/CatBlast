use bevy::prelude::*;
use crate::plugins::match3::components::{FallAnimating, Gem, GemType, GridPosition, OnMatch3Scene};
use crate::plugins::match3::resources::{Board, GemAtlas, Match3Config};

pub fn clear_all_gems(commands: &mut Commands, q_gems: &Query<Entity, With<Gem>>) {
    for entity in q_gems.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_new_board(
    commands: &mut Commands,
    board: &Board,
    gem_atlas: &GemAtlas,
    config: &Match3Config,
) {
    // 先在棋盘上方生成宝石（用于下落动画）
    let spawn_offset_y = board.height; // 在棋盘上方 board.height 个格子的位置生成

    for x in 0..board.width {
        for y in 0..board.height {
            let gem_type = GemType::random();
            let atlas_index = gem_type as usize + 8;

            // 计算最终的世界坐标（宝石应该在的位置）
            let final_world_x = x as f32 * config.gem_size
                - (board.width as f32 * config.gem_size) / 2.0
                + config.gem_size / 2.0;
            let final_world_y = y as f32 * config.gem_size
                - (board.height as f32 * config.gem_size) / 2.0
                + config.gem_size / 2.0;

            // 计算起始的世界坐标（在棋盘上方）
            let start_world_y = (y + spawn_offset_y) as f32 * config.gem_size
                - (board.height as f32 * config.gem_size) / 2.0
                + config.gem_size / 2.0;

            commands.spawn((
                Sprite::from_atlas_image(
                    gem_atlas.image.clone(),
                    TextureAtlas {
                        layout: gem_atlas.layout.clone(),
                        index: atlas_index,
                    },
                ),
                Transform::from_xyz(final_world_x, start_world_y, 0.0),
                Gem,
                OnMatch3Scene,
                GridPosition { x, y },
                gem_type,
                // 添加下落动画，让宝石从上方掉下来
                FallAnimating::new(
                    Vec2::new(final_world_x, start_world_y), // start_pos: 起始位置（上方）
                    Vec2::new(final_world_x, final_world_y), // end_pos: 最终位置
                    0.5 + (y as f32 * 0.05), // 根据高度设置不同的持续时间，制造波浪效果
                ),
            ));
        }
    }
}