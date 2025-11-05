use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub score: u32,
}

#[derive(Resource, Default)]
pub struct Combo {
    pub times: u32,
}

