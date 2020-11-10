use crate::world;

pub const WIDTH: f32 = 50.0;
pub const HEIGHT: f32 = 50.0;

pub const INITIAL_POSITION_X: f32 = -(world::SCREEN_WIDTH as f32 / 2.0) + 128.0;
pub const INITIAL_POSITION_Y: f32 = HEIGHT / 2.0;

pub const VELOCITY_ON_JUMP: f32 = 400.0;
pub const MOVEMENT_VELOCITY: f32 = 400.0;

#[derive(Debug)]
pub struct Player {
    pub movement_state: PlayerMovementState,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerMovementState {
    Staying,
    Running,
    Jumping,
}

impl Player {
    pub fn new() -> Self {
        Player {
            movement_state: PlayerMovementState::Staying,
        }
    }
}
