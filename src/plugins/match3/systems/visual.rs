use crate::plugins::match3::components::{Gem, Selected};
use bevy::prelude::*;

const SELECTED_SCALE: f32 = 1.15; // 选中时放大 15%
const NORMAL_SCALE: f32 = 1.0;
const SELECTED_Z: f32 = 10.0; // 选中时 Z 轴提升，确保在最上层
const NORMAL_Z: f32 = 0.0;

const HIGHLIGHT_COLOR: Color = Color::linear_rgba(1.5, 1.5, 1.5, 1.0);

pub fn apply_selection_effect(mut query: Query<(&mut Transform, &mut Sprite), Added<Selected>>) {
    for (mut transform, mut sprite) in query {
        println!("Applying selection effect");
        transform.scale = Vec3::splat(SELECTED_SCALE);
        transform.translation.z = SELECTED_Z;
        sprite.color = HIGHLIGHT_COLOR;
    }
}

pub fn remove_selection_effect(
    mut removed: RemovedComponents<Selected>,
    mut query: Query<(&mut Transform, &mut Sprite)>,
) {
    for entity in removed.read() {
        if let Ok((mut transform, mut sprite)) = query.get_mut(entity) {
            println!("Removing selection effect.");
            transform.scale = Vec3::splat(NORMAL_SCALE);
            transform.translation.z = NORMAL_Z;
            sprite.color = Color::WHITE;
        }
    }
}
