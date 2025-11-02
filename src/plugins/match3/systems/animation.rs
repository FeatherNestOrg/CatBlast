use crate::plugins::match3::components::{spawn_blast_particles, BlastAnimating, BlastParticle, SwapAnimating};
use crate::plugins::match3::state::Match3State;
use bevy::prelude::*;
use bevy::time::Time;

pub fn swap_animation_system(
    time: Res<Time>,
    mut q_animation_gems: Query<(Entity, &mut Transform, &mut SwapAnimating)>,
    mut commands: Commands,
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

pub fn check_swap_animation_completion_system(
    q_animating_swap: Query<&SwapAnimating>,
    mut next_state: ResMut<NextState<Match3State>>,
) {
    if q_animating_swap.is_empty() {
        println!("All animations finished. Switching to ProcessingMatches state.");
        next_state.set(Match3State::ProcessingMatches);
    }
}

pub fn check_blast_animation_completion_system(
    q_animating_blast: Query<&BlastAnimating>,
    q_blast_particles: Query<&BlastParticle>,
    mut next_state: ResMut<NextState<Match3State>>,
) {
    if q_animating_blast.is_empty() && q_blast_particles.is_empty() {
        println!("All animations finished. Switching to ProcessingMatches state.");
        next_state.set(Match3State::ProcessingMatches);
    }
}
