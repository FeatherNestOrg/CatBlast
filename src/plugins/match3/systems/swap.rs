use crate::plugins::match3::components::{SwapAnimating, GridPosition};
use crate::plugins::match3::message::RequestSwapEvent;
use crate::plugins::match3::resources::{Board, PendingSwap};
use crate::plugins::match3::state::Match3State;
use bevy::prelude::{Commands, Entity, MessageReader, NextState, Query, ResMut, Transform};

pub fn swap_system(
    mut swap_mr: MessageReader<RequestSwapEvent>,
    mut q_gems_pos: Query<&mut GridPosition>,
    mut q_gems_trans: Query<&mut Transform>,
    mut board: ResMut<Board>,
    mut pending_swap: ResMut<PendingSwap>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<Match3State>>,
) {
    if swap_mr.is_empty() {
        return;
    }


    for event in swap_mr.read() {
        let e1 = event.entity1;
        let e2 = event.entity2;


        logical_swap(&mut board, &mut q_gems_pos, e1, e2);
        add_swap_animation(&mut commands, &mut q_gems_trans, e1, e2);

        pending_swap.entities = Some((e1, e2));
        next_state.set(Match3State::Animating);
    }
    swap_mr.clear();
}

pub fn logical_swap(board: &mut Board, q_gems_pos: &mut Query<&mut GridPosition>, e1: Entity, e2: Entity) {
    if let Ok([mut pos1, mut pos2]) = q_gems_pos.get_many_mut([e1, e2]) {
        std::mem::swap(&mut *pos1, &mut *pos2);
        board.grid[pos1.x as usize][pos1.y as usize] = Some(e1);
        board.grid[pos2.x as usize][pos2.y as usize] = Some(e2);
    }
}

pub fn add_swap_animation(commands: &mut Commands, q_gems_trans: &mut Query<&mut Transform>, e1: Entity, e2: Entity) {
    if let Ok([trans1, trans2]) =
        q_gems_trans.get_many_mut([e1, e2])
    {
        let anim1 = SwapAnimating::new(trans1.translation, trans2.translation, 0.2);
        let anim2 = SwapAnimating::new(trans2.translation, trans1.translation, 0.2);

        commands.entity(e1).insert(anim1);
        commands.entity(e2).insert(anim2);
    };
}