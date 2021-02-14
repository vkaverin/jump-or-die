use crate::world;
use bevy::core::Timer;

pub const ENEMY_WIDTH: f32 = 50.0;
pub const ENEMY_HEIGHT: f32 = 50.0;

pub const VELOCITY_X: f32 = 200.0;
pub const VELOCITY_Y: f32 = 0.0;

pub const INITIAL_POSITION_X: f32 = (world::SCREEN_WIDTH + ENEMY_WIDTH) / 2.0;
pub const INITIAL_POSITION_Y: f32 = ENEMY_HEIGHT / 2.0;

pub const SCORE: f32 = 50.0;

pub struct Enemy;

pub struct SpawnTimer {
    pub timer: Timer,
}
