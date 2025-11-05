use bevy::math::{i32};
use crate::plugins::match3::components::{GridPosition, Selected};
use crate::plugins::match3::message::RequestSwapEvent;
use crate::plugins::match3::resources::{Board, PendingSwap};
use crate::plugins::match3::state::Match3State;
use bevy::prelude::*;
use bevy::sprite::Sprite;
use crate::plugins::match3::components::animation::SwapAnimating;

pub fn swap_system(
    mut swap_mr: MessageReader<RequestSwapEvent>,
    mut q_gems_pos: Query<&mut GridPosition>,
    mut q_gems_trans: Query<&mut Transform>,
    mut q_gems_sprite: Query<&mut Sprite>,
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

        if let Ok(mut sprite) = q_gems_sprite.get_mut(e1) {
            sprite.color = Color::WHITE;
        }
        if let Ok(mut trans) = q_gems_trans.get_mut(e1) {
            trans.scale = Vec3::splat(1.0);
        }

        // TODO: better handle with selected visual effect.


        pending_swap.entities = Some((e1, e2));
        commands.entity(e1).remove::<Selected>();
        next_state.set(Match3State::Animating);
    }
    swap_mr.clear();
}
pub fn check_swap(board: &mut Board, q_gems_pos: &mut Query<&mut GridPosition>, e1: Entity, e2: Entity) -> bool {
    if let Ok([pos1, pos2]) = q_gems_pos.get_many_mut([e1, e2]) {
        let dx = i32::abs(pos1.x as i32 - pos2.x as i32);
        let dy = i32::abs(pos1.y as i32 - pos2.y as i32);
        return (dx == 1 && dy == 0) || (dx == 0 && dy == 1);
    }
    false
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