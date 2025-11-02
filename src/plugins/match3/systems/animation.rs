use crate::plugins::match3::components::GemAnimating;
use crate::plugins::match3::state::Match3State;
use bevy::prelude::{Commands, Entity, NextState, Query, Res, ResMut, Transform};
use bevy::time::Time;

pub fn gem_animation_system(
    time: Res<Time>,
    mut q_animation_gems: Query<(Entity, &mut Transform, &mut GemAnimating)>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut animation) in q_animation_gems.iter_mut() {
        animation.timer.tick(time.delta());

        let t = animation.timer.fraction();
        transform.translation = animation.start_pos.lerp(animation.end_pos, t);
        if animation.timer.just_finished() {
            transform.translation = animation.end_pos;
            commands.entity(entity).remove::<GemAnimating>();
        }
    }
}

pub fn check_animation_completion_system(
    q_animating_gems: Query<&GemAnimating>,
    mut next_state: ResMut<NextState<Match3State>>,
) {
    if q_animating_gems.is_empty() {
        println!("All animations finished. Switching to ProcessingMatches state.");
        next_state.set(Match3State::ProcessingMatches);
    }
}
