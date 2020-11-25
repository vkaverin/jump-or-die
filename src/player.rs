use crate::world;
use crate::world::Velocity;
use crate::awards::Award;

pub const WIDTH: f32 = 50.0;
pub const HEIGHT: f32 = 50.0;

pub const INITIAL_POSITION_X: f32 = -(world::SCREEN_WIDTH as f32 / 2.0) + 128.0;
pub const INITIAL_POSITION_Y: f32 = HEIGHT / 2.0;

pub const VELOCITY_ON_JUMP: f32 = 600.0;
pub const MOVEMENT_VELOCITY: f32 = 400.0;

#[derive(Debug)]
pub struct Player {
    pub movement_state: PlayerMovementState,
    pub health: u8,
    pub max_health: u8,
}

impl Player {
    pub fn new() -> Self {
        Player {
            movement_state: PlayerMovementState::Staying,
            health: 3,
            max_health: 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerMovementState {
    Staying,
    Running,
    Jumping,
    Falling,
}

pub enum PlayerEvent {
    Hit,
    Award(Award),
}

pub fn update_movement_state(player: &mut Player, velocity: &Velocity) {
    if velocity.0.y() > 0.0 {
        player.movement_state = PlayerMovementState::Jumping;
    } else if velocity.0.y() < 0.0 {
        player.movement_state = PlayerMovementState::Falling;
    } else if velocity.0.x() != 0.0 {
        player.movement_state = PlayerMovementState::Running
    } else {
        player.movement_state = PlayerMovementState::Staying;
    }
}
