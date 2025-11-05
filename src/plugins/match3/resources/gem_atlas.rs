use bevy::prelude::*;

#[derive(Resource)]
pub struct GemAtlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
}

