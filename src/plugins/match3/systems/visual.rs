use crate::plugins::match3::components::{Gem, Selected};
use bevy::prelude::*;

const SELECTED_SCALE: f32 = 1.15; // 选中时放大 15%
const NORMAL_SCALE: f32 = 1.0;
const SELECTED_Z: f32 = 10.0; // 选中时 Z 轴提升，确保在最上层
const NORMAL_Z: f32 = 0.0;

const BRIGHT: f32 = 1.5;

pub fn animate_selection_effect(
    query: Query<(&mut Transform, &mut Sprite, &mut Selected)>,
    time: Res<Time>,
) {
    for (mut transform, mut sprite, mut selected) in query {
        if selected.timer.just_finished() {
            return;
        }
        selected.timer.tick(time.delta());
        let progress = selected.timer.fraction();
        let scale = 1.0 + (SELECTED_SCALE - 1.0) * progress;
        transform.scale = Vec3::splat(scale);
        let brightness = 1.0 + (BRIGHT - 1.0) * progress;
        sprite.color = Color::linear_rgba(brightness, brightness, brightness, 1.0);
        transform.translation.z = SELECTED_Z;
    }
}

pub fn apply_selection_effect(query: Query<(&mut Transform, &mut Sprite), Added<Selected>>) {
    for (mut transform, mut sprite) in query {
        tracing::debug!("Applying selection effect");
        transform.translation.z = 10.0;
        sprite.color = Color::WHITE;
    }
}

pub fn remove_selection_effect(
    mut removed: RemovedComponents<Selected>,
    mut query: Query<(&mut Transform, &mut Sprite)>,
) {
    for entity in removed.read() {
        if let Ok((mut transform, mut sprite)) = query.get_mut(entity) {
            tracing::debug!("Removing selection effect.");
            transform.scale = Vec3::splat(NORMAL_SCALE);
            transform.translation.z = NORMAL_Z;
            sprite.color = Color::WHITE;
        }
    }
}
