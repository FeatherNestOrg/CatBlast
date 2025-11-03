use crate::plugins::match3::components::{Gem, GemType, GridPosition};
use crate::plugins::match3::resources::Board;
use bevy::prelude::*;
pub fn has_possible_move(
    board: &Board,
    q_gems: &Query<(Entity, &GridPosition, &GemType), With<Gem>>,
) -> bool {
    // 1. 预处理：将实体查询数据转换为一个快速访问的二维网格
    let mut type_grid: Vec<Vec<Option<GemType>>> =
        vec![vec![None; board.height as usize]; board.width as usize];

    for (_, grid_pos, gem_type) in q_gems.iter() {
        // 检查边界，以防万一
        if grid_pos.x < board.width && grid_pos.y < board.height {
            type_grid[grid_pos.x as usize][grid_pos.y as usize] = Some(*gem_type);
        }
    }

    // 2. 遍历所有可能的交换
    for y in 0..board.height {
        for x in 0..board.width {
            // 如果当前格子是空的，不能从这里发起交换
            if type_grid[x as usize][y as usize].is_none() {
                continue;
            }

            // 只需检查“右边”和“上边”的邻居，避免重复检查
            // (检查 a-b 和 b-a 是一样的)

            // 检查水平交换 (与右边的宝石)
            if x < board.width - 1 {
                if type_grid[x as usize + 1][y as usize].is_some() {
                    if check_swap_for_match(&type_grid, (x, y), (x + 1, y)) {
                        return true; // 找到了一个可行解！立即返回。
                    }
                }
            }

            // 检查垂直交换 (与上边的宝石)
            if y < board.height - 1 {
                if type_grid[x as usize][y as usize + 1].is_some() {
                    if check_swap_for_match(&type_grid, (x, y), (x, y + 1)) {
                        return true; // 找到了一个可行解！立即返回。
                    }
                }
            }
        }
    }

    // 遍历完所有可能的交换都没有找到解
    false
}
fn check_swap_for_match(
    type_grid: &Vec<Vec<Option<GemType>>>,
    pos1: (u32, u32),
    pos2: (u32, u32),
) -> bool {
    let (x1, y1) = pos1;
    let (x2, y2) = pos2;

    // 获取两个位置上的宝石类型
    let type1 = type_grid[x1 as usize][y1 as usize].unwrap();
    let type2 = type_grid[x2 as usize][y2 as usize].unwrap();

    // 如果两个宝石类型相同，交换它们不可能形成新的匹配
    if type1 == type2 {
        return false;
    }

    // 检查：如果把 type2 放到 pos1，是否会在 pos1 周围形成匹配？
    if check_match_patterns_at(type_grid, pos1, type2, pos2) {
        return true;
    }

    // 检查：如果把 type1 放到 pos2，是否会在 pos2 周围形成匹配？
    if check_match_patterns_at(type_grid, pos2, type1, pos1) {
        return true;
    }

    false
}

fn check_match_patterns_at(
    grid: &Vec<Vec<Option<GemType>>>,
    pos: (u32, u32),
    new_type: GemType,
    swapped_from_pos: (u32, u32),
) -> bool {
    let (width, height) = (grid.len() as u32, grid[0].len() as u32);
    let (px, py) = pos;

    // 一个辅助函数，用于安全地获取模拟后的网格中的类型
    let get_type = |x: u32, y: u32| -> Option<GemType> {
        if (x, y) == swapped_from_pos {
            // 这个位置的宝石已经被“换走”了，所以我们不考虑它
            None
        } else {
            grid.get(x as usize)
                .and_then(|row| row.get(y as usize))
                .and_then(|&t| t)
        }
    };

    // --- 检查水平匹配 ---
    // 模式: [X, new_type, X] (中间)
    if px > 0 && px < width - 1 {
        if get_type(px - 1, py) == Some(new_type) && get_type(px + 1, py) == Some(new_type) {
            return true;
        }
    }
    // 模式: [new_type, X, X] (左边)
    if px < width - 2 {
        if get_type(px + 1, py) == Some(new_type) && get_type(px + 2, py) == Some(new_type) {
            return true;
        }
    }
    // 模式: [X, X, new_type] (右边)
    if px > 1 {
        if get_type(px - 1, py) == Some(new_type) && get_type(px - 2, py) == Some(new_type) {
            return true;
        }
    }

    // --- 检查垂直匹配 ---
    // 模式: [X, new_type, X] (中间)
    if py > 0 && py < height - 1 {
        if get_type(px, py - 1) == Some(new_type) && get_type(px, py + 1) == Some(new_type) {
            return true;
        }
    }
    // 模式: [new_type, X, X] (下边)
    if py < height - 2 {
        if get_type(px, py + 1) == Some(new_type) && get_type(px, py + 2) == Some(new_type) {
            return true;
        }
    }
    // 模式: [X, X, new_type] (上边)
    if py > 1 {
        if get_type(px, py - 1) == Some(new_type) && get_type(px, py - 2) == Some(new_type) {
            return true;
        }
    }

    false
}