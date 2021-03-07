use crate::awards::Award;
use crate::player;
use bevy::math::Vec2;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;

pub const GRAVITY: f32 = player::VELOCITY_ON_JUMP * 4.0;

#[derive(Default, Copy, Clone)]
pub struct Velocity(pub Vec2);

pub struct Gravity(pub f32);

impl Default for Gravity {
    fn default() -> Self {
        Gravity(GRAVITY)
    }
}

pub struct AffectedByGravity;

#[derive(Default)]
pub struct Deformation {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub enum Collider {
    Solid,
    Award(Award),
}
