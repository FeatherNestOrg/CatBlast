pub(crate) use crate::plugins::ui::overlays::components::OverlayBackgroundMarker;
use crate::plugins::ui::overlays::settings::SettingsPlugin;
use bevy::prelude::*;

pub mod components;
pub mod settings;

pub struct OverlayPlugin;
impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SettingsPlugin);
    }
}
pub fn setup_overlay_background(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100.0),
            height: percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        ZIndex(1),
        OverlayBackgroundMarker,
    ));
}

pub fn cleanup_overlay_background(
    mut commands: Commands,
    query: Query<Entity, With<OverlayBackgroundMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
