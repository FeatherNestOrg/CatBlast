use bevy::prelude::*;


pub struct Skill {}
#[derive(Component)]
pub struct Player {
    pub hp: i32,
}

#[derive(Component)]
pub struct Enemy {
    pub hp: i32,
}