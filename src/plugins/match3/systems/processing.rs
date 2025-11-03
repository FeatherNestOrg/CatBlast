use crate::plugins::match3::components::{BlastAnimating, Gem, GemType, GridPosition};
use crate::plugins::match3::resources::{Board, Match3Config, PendingSwap};
use crate::plugins::match3::state::Match3State;
use crate::plugins::match3::systems::swap::{add_swap_animation, logical_swap};
use bevy::prelude::*;
use std::collections::HashSet;
use crate::plugins::match3::systems::gravity::apply_gravity;

pub fn process_board_state_system(
    mut commands: Commands,
    mut param_set: ParamSet<(
        Query<(Entity, &GemType, &mut GridPosition), With<Gem>>,
        Query<&mut GridPosition>,
        Query<&mut Transform>,
        Query<(Entity, &mut GridPosition), With<Gem>>,
    )>,
    mut board: ResMut<Board>,
    config: Res<Match3Config>,
    mut pending_swap: ResMut<PendingSwap>,
    mut next_state: ResMut<NextState<Match3State>>,
) {
    let matches_to_remove = find_all_matches_on_board(&board, &param_set.p0());

    if !matches_to_remove.is_empty() {
        pending_swap.entities = None;
        for entity in &matches_to_remove {
            commands.entity(*entity).insert(BlastAnimating::new(0.3));
        }
        next_state.set(Match3State::Animating)
    } else if let Some((e1, e2)) = pending_swap.entities.take() {
        tracing::info!("No matches found, and there was a pending swap. Reverting.");
        logical_swap(&mut board, &mut param_set.p1(), e1, e2);
        add_swap_animation(&mut commands, &mut param_set.p2(), e1, e2);
        next_state.set(Match3State::Animating);
    } else {
        let has_gravity = apply_gravity(&mut commands, &mut param_set.p3(), &board, &config);
        if has_gravity {
            info!("Applying gravity, gems falling.");
            next_state.set(Match3State::Animating);
        } else {
            next_state.set(Match3State::AwaitingInput)
        }
    }
}

pub fn find_all_matches_on_board(
    board: &Board,
    q_gems: &Query<(Entity, &GemType, &mut GridPosition), With<Gem>>,
) -> HashSet<Entity> {
    let mut matches_to_remove: HashSet<Entity> = HashSet::new();

    let mut gem_map: Vec<Vec<Option<(Entity, GemType)>>> =
        vec![vec![None; board.height as usize]; board.width as usize];

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
    matches_to_remove
}
