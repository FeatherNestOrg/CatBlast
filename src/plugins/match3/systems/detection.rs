use std::collections::HashSet;
use crate::plugins::match3::state::Match3State;
use bevy::prelude::*;
use crate::plugins::match3::components::{BlastAnimating, Gem, GemType, GridPosition};
use crate::plugins::match3::resources::Board;

pub fn match_detection_system(
    mut commands: Commands,
    q_gems: Query<(Entity, &GemType, &GridPosition), With<Gem>>,
    board: Res<Board>,
    mut next_state: ResMut<NextState<Match3State>>,
) {
    let mut matches_to_remove: HashSet<Entity> = HashSet::new();

    let mut gem_map: Vec<Vec<Option<(Entity, GemType)>>> = vec![vec![None; board.height as usize]; board.width as usize];

    for (entity, gem_type, pos) in q_gems.iter() {
        gem_map[pos.x as usize][pos.y as usize] = Some((entity, *gem_type));
    }

    for y in 0..board.height {
        for x in 0..board.width {
            if let Some((_, gem_type)) = gem_map[x as usize][y as usize] {
                let mut match_count = 1;
                let mut match_x = x + 1;

                // 向右检查相同类型的宝石
                while match_x < board.width {
                    if let Some((_, next_type)) = gem_map[match_x as usize][y as usize] {
                        if next_type == gem_type {
                            match_count += 1;
                            match_x += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                // 如果找到3个或以上匹配，标记为移除
                if match_count >= 3 {
                    for i in 0..match_count {
                        if let Some((entity, _)) = gem_map[(x + i) as usize][y as usize] {
                            matches_to_remove.insert(entity);
                        }
                    }
                }
            }
        }
    }

    // 检测竖直匹配
    for x in 0..board.width {
        for y in 0..board.height {
            if let Some((_, gem_type)) = gem_map[x as usize][y as usize] {
                let mut match_count = 1;
                let mut match_y = y + 1;

                // 向上检查相同类型的宝石
                while match_y < board.height {
                    if let Some((_, next_type)) = gem_map[x as usize][match_y as usize] {
                        if next_type == gem_type {
                            match_count += 1;
                            match_y += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                // 如果找到3个或以上匹配，标记为移除
                if match_count >= 3 {
                    for i in 0..match_count {
                        if let Some((entity, _)) = gem_map[x as usize][(y + i) as usize] {
                            matches_to_remove.insert(entity);
                        }
                    }
                }
            }
        }
    }
    for entity in &matches_to_remove {
        commands.entity(*entity).insert(BlastAnimating::new(0.3));
    }
    if !matches_to_remove.is_empty() {
        next_state.set(Match3State::BlastAnimating)
    } else {
        next_state.set(Match3State::AwaitingInput)
    }
}

