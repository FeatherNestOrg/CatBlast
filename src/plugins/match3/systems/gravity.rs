use bevy::prelude::*;
use crate::plugins::match3::components::{FallAnimating, Gem, GridPosition};
use crate::plugins::match3::resources::{Board, Match3Config};

pub fn apply_gravity(
    commands: &mut Commands,
    q_gems: &mut Query<(Entity, &mut GridPosition), With<Gem>>,
    board: &Board,
    config: &Match3Config,
) -> bool {
    let mut gem_map: Vec<Vec<Option<Entity>>> =
        vec![vec![None; board.height as usize]; board.width as usize];
    for (entity, pos) in q_gems.iter() {
        gem_map[pos.x as usize][pos.y as usize] = Some(entity);
    }
    let mut has_moved = false;
    for x in 0..board.width {
        let mut write_y = 0; // 写入位置（从下往上填充）

        for y in 0..board.height {
            if let Some(entity) = gem_map[x as usize][y as usize] {
                if write_y != y {
                    // 需要下落
                    if let Ok((_, mut pos)) = q_gems.get_mut(entity) {
                        // 计算世界坐标
                        let world_x = x as f32 * config.gem_size
                            - (board.width as f32 * config.gem_size) / 2.0
                            + config.gem_size / 2.0;
                        let from_y = y as f32 * config.gem_size
                            - (board.height as f32 * config.gem_size) / 2.0
                            + config.gem_size / 2.0;
                        let to_y = write_y as f32 * config.gem_size
                            - (board.height as f32 * config.gem_size) / 2.0
                            + config.gem_size / 2.0;

                        pos.y = write_y;
                        commands.entity(entity).insert(FallAnimating::new(
                            Vec2::new(world_x, from_y),
                            Vec2::new(world_x, to_y),
                            0.3,
                        ));
                        has_moved = true;
                    }
                }
                write_y += 1;
            }
        }
    }

    has_moved
}