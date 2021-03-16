use crate::awards::Award;
use crate::player;
use bevy::prelude::Vec2;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;

pub const GRAVITY: f32 = player::VELOCITY_ON_JUMP * 4.0;

#[derive(Debug, Default, Copy, Clone)]
pub struct Velocity {
    base: Vec2,
    boost: Vec2,
}

impl Velocity {
    pub fn new(v: Vec2) -> Self {
        Self {
            base: v,
            boost: Vec2::ONE,
        }
    }

    pub fn with_horizontal(v: f32) -> Self {
        Self::new(Vec2::new(v, 0.0))
    }

    pub fn current(&self) -> Vec2 {
        self.base * self.boost
    }

    pub fn set_current(&mut self, v: Vec2) {
        self.base = v;
        self.drop_boost();
    }

    pub fn horizontal(&self) -> f32 {
        self.current().x
    }

    pub fn add_horizontal(&mut self, v: f32) {
        self.base.x += v;
    }

    pub fn set_horizontal(&mut self, v: f32) {
        self.base.x = v;
        self.boost.x = 1.0;
    }

    pub fn drop_horizontal(&mut self) {
        self.set_horizontal(0.0)
    }

    pub fn vertical(&self) -> f32 {
        self.current().y
    }

    pub fn set_vertical(&mut self, v: f32) {
        self.base.y = v;
        self.boost.y = 1.0;
    }

    pub fn drop_vertical(&mut self) {
        self.set_vertical(0.0)
    }

    pub fn add_vertical(&mut self, v: f32) {
        self.base.y += v;
    }

    pub fn set_boost(&mut self, v: Vec2) {
        self.boost = v;
    }

    pub fn drop_boost(&mut self) {
        self.boost = Vec2::ONE;
    }

    pub fn reset(&mut self) {
        self.base = Vec2::ZERO;
        self.boost = Vec2::ONE;
    }
}

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
