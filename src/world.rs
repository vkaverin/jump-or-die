use crate::player;
use bevy::math::Vec2;
use crate::enemies::Award;

pub const SCREEN_WIDTH: u32 = 1280;
pub const SCREEN_HEIGHT: u32 = 720;

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

pub enum Collider {
    Solid,
    Award(Award),
}
