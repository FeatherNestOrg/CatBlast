use crate::plugins::match3::components::{GemAnimating, GridPosition};
use crate::plugins::match3::message::RequestSwapEvent;
use crate::plugins::match3::resources::Board;
use crate::plugins::match3::state::Match3State;
use bevy::prelude::{Commands, MessageReader, NextState, Query, ResMut, Transform};

pub fn swap_system(
    mut swap_mr: MessageReader<RequestSwapEvent>,
    mut q_gems: Query<(&mut GridPosition, &mut Transform)>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<Match3State>>,
) {
    if swap_mr.is_empty() {
        return;
    }
    for event in swap_mr.read() {
        if let Ok([(mut pos1, trans1), (mut pos2, trans2)]) =
            q_gems.get_many_mut([event.entity1, event.entity2])
        {
            let anim1 = GemAnimating::new(trans1.translation, trans2.translation, 0.2);
            let anim2 = GemAnimating::new(trans2.translation, trans1.translation, 0.2);

            println!("Swapping gems at {:?} and {:?}", *pos1, *pos2);

            std::mem::swap(&mut *pos1, &mut *pos2);

            board.grid[pos1.x as usize][pos1.y as usize] = Some(event.entity1);
            board.grid[pos2.x as usize][pos2.y as usize] = Some(event.entity2);

            commands.entity(event.entity1).insert(anim1);
            commands.entity(event.entity2).insert(anim2);

            println!("Starting swap animation, switching to Animating state.");
            next_state.set(Match3State::Animating);
        };
    }
    swap_mr.clear();
}
