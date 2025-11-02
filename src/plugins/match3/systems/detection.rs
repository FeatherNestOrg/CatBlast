use crate::plugins::match3::components::GridPosition;
use crate::plugins::match3::message::{RequestSwapEvent, SwapCompletedEvent};
use crate::plugins::match3::resources::Board;
use bevy::prelude::*;

pub fn swap_system(
    mut swap_mr: MessageReader<RequestSwapEvent>,
    mut complete_mw: MessageWriter<SwapCompletedEvent>,
    mut q_gems: Query<(&mut GridPosition, &mut Transform)>,
    mut board: ResMut<Board>,
) {
    for event in swap_mr.read() {
        if let Ok([gem1, gem2]) = q_gems.get_many_mut([event.entity1, event.entity2]) {
            let (mut pos1, mut trans1) = gem1;
            let (mut pos2, mut trans2) = gem2;

            println!("Swapping gems at {:?} and {:?}", *pos1, *pos2);

            std::mem::swap(&mut *pos1, &mut *pos2);
            std::mem::swap(&mut trans1.translation, &mut trans2.translation);

            board.grid[pos1.x as usize][pos1.y as usize] = Some(event.entity1);
            board.grid[pos2.x as usize][pos2.y as usize] = Some(event.entity2);

            complete_mw.write(SwapCompletedEvent {
                entity1: event.entity2,
                entity2: event.entity1,
            });
        };
    }
}

pub fn match_detection_system() {
    /* ... */
}
