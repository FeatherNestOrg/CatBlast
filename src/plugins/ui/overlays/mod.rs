use crate::plugins::ui::overlays::settings::SettingsPlugin;
use bevy::prelude::*;

mod components;
pub use components::OverlayBackgroundMarker;
mod messages;
pub mod settings;
use crate::state::OverlayState;
pub use messages::{OverlayAction, OverlayMessage};

pub struct OverlayPlugin;
impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<OverlayMessage>()
            .add_plugins(SettingsPlugin);
    }
}
pub fn setup_overlay_background(commands: &mut Commands) {
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
    commands: &mut Commands,
    query: Query<Entity, With<OverlayBackgroundMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
