use bevy::prelude::*;
use bevy::time::TimerMode::Once;
use rand::Rng;

#[derive(Component)]
pub struct Gem;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum GemType {
    Ice,
    Water,
    Fairy,
    Poison,
    Fight,
    Fire,
    Grass,
    Normal,
}

impl GemType {
    pub fn random() -> Self {
        const VARIANTS: [GemType; 8] = [
            GemType::Ice,
            GemType::Water,
            GemType::Fairy,
            GemType::Poison,
            GemType::Fight,
            GemType::Fire,
            GemType::Grass,
            GemType::Normal,
        ];
        let mut rng = rand::rng();
        let idx = rng.random_range(0..VARIANTS.len());
        VARIANTS[idx]
    }
}

#[derive(Component)]
pub struct SwapAnimating {
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    pub timer: Timer,
}

impl SwapAnimating {
    pub fn new(start_pos: Vec3, end_pos: Vec3, duration_secs: f32) -> Self {
        Self {
            start_pos,
            end_pos,
            timer: Timer::from_seconds(duration_secs, Once),
        }
    }
}

#[derive(Component)]
pub struct BlastAnimating {
    pub timer: Timer,
}

impl BlastAnimating {
    pub fn new(duration_secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration_secs, Once),
        }
    }
}

#[derive(Component)]
pub struct BlastParticle {
    pub velocity: Vec3,
    pub lifetime: Timer,
}

impl BlastParticle {
    pub fn new(velocity: Vec3, lifetime_secs: f32) -> Self {
        Self {
            velocity,
            lifetime: Timer::from_seconds(lifetime_secs, Once),
        }
    }
}

pub fn spawn_blast_particles(
    commands: &mut Commands,
    position: Vec3,
    gem_sprite: &Sprite,
) {
    let particle_count = 12;
    for i in 0..particle_count {
        let angle = (i as f32 / particle_count as f32) * std::f32::consts::TAU;
        let speed = 150.0;
        let velocity = Vec3::new(
            angle.cos() * speed,
            angle.sin() * speed,
            0.0,
        );
        commands.spawn((
            Sprite {
                image: gem_sprite.image.clone(),
                texture_atlas: gem_sprite.texture_atlas.clone(),
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..default()
            },
            Transform::from_translation(position),
            BlastParticle::new(velocity, 0.4),
        ));
    }
}

#[derive(Component)]
pub struct Selected {
    pub timer: Timer,
}

impl Selected {
    pub fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, Once),
        }
    }
}
