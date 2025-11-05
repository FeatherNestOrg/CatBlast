use crate::plugins::match3::components::animation::{spawn_blast_particles, BlastAnimating, BlastParticle, FallAnimating, SwapAnimating};
use crate::plugins::match3::state::Match3State;
use bevy::prelude::*;
use bevy::time::Time;
use crate::plugins::match3::resources::PendingSwap;
use crate::plugins::match3::state::Match3State::ProcessingBoard;

pub fn swap_animation_system(
    time: Res<Time>,
    mut q_animation_gems: Query<(Entity, &mut Transform, &mut SwapAnimating)>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<Match3State>>,
    pending_swap: Res<PendingSwap>,
) {
    for (entity, mut transform, mut animation) in q_animation_gems.iter_mut() {
        animation.timer.tick(time.delta());

        let t = animation.timer.fraction();
        transform.translation = animation.start_pos.lerp(animation.end_pos, t);
        if animation.timer.just_finished() {
            transform.translation = animation.end_pos;
            commands.entity(entity).remove::<SwapAnimating>();
        }
    }
}

pub fn blast_animation_system(
    time: Res<Time>,
    q_blast_gems: Query<(Entity, &mut Sprite, &mut BlastAnimating, &Transform)>,
    mut commands: Commands,
) {
    for (entity, mut sprite, mut animation, transform) in q_blast_gems {
        animation.timer.tick(time.delta());

        let t = animation.timer.fraction();
        sprite.color.set_alpha(1.0 - t);
        if t < 0.1 {
            spawn_blast_particles(&mut commands, transform.translation, &sprite);
        }
        if animation.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn blast_particle_system(
    time: Res<Time>,
    mut q_particles: Query<(Entity, &mut Transform, &mut Sprite, &mut BlastParticle)>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut sprite, mut particle) in q_particles.iter_mut() {
        particle.lifetime.tick(time.delta());
        transform.translation += particle.velocity * time.delta_secs();

        let t = particle.lifetime.fraction();
        sprite.color.set_alpha((1.0 - t) * 0.8);
        if particle.lifetime.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn falling_animation_system(
    mut commands: Commands,
    mut q_falling: Query<(Entity, &mut Transform, &mut FallAnimating)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut fall) in q_falling {
        fall.timer.tick(time.delta());
        let progress = fall.timer.fraction();
        let eased_progress = 1.0 - (1.0 - progress).powi(3);

        // 使用缓动进度计算当前位置
        transform.translation.x = fall.start_pos.x + (fall.end_pos.x - fall.start_pos.x) * eased_progress;
        transform.translation.y = fall.start_pos.y + (fall.end_pos.y - fall.start_pos.y) * eased_progress;

        if fall.timer.just_finished() {
            transform.translation.x = fall.end_pos.x;
            transform.translation.y = fall.end_pos.y;
            commands.entity(entity).remove::<FallAnimating>();
        }
    }
}

pub fn check_animation_system(
    q_swap: Query<&SwapAnimating>,
    q_blast: Query<&BlastAnimating>,
    q_particle: Query<&BlastParticle>,
    q_fall: Query<&FallAnimating>,
    mut next_state: ResMut<NextState<Match3State>>,
) {
    if q_swap.is_empty() && q_blast.is_empty() && q_particle.is_empty() && q_fall.is_empty() {
        next_state.set(ProcessingBoard);
    }
}