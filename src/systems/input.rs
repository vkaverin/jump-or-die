use crate::game::{Game, GameState};
use crate::player::{self, Player, PlayerMovementState};
use crate::world::Velocity;
use bevy::prelude::*;

pub fn input(
    input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut player: Mut<Player>,
    mut velocity: Mut<Velocity>,
    mut transform: Mut<Transform>,
) {
    match game.state {
        GameState::WaitingForStart => {
            if input.pressed(KeyCode::Space) {
                game.state = GameState::Running;
            }
        }
        GameState::Running => {
            input_on_running_game(&input, &mut game, &mut player, &mut velocity, &mut transform);
        }
        GameState::Paused => {
            input_on_paused_game(&input, &mut game);
        }
        GameState::GameOver => {}
    }
}

fn input_on_running_game(
    input: &Res<Input<KeyCode>>,
    mut game: &mut ResMut<Game>,
    mut player: &mut Mut<Player>,
    mut velocity: &mut Mut<Velocity>,
    mut transform: &mut Mut<Transform>,
) {
    if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::P){
        game.state = GameState::Paused;
        return;
    }

    if input.pressed(KeyCode::R) {
        velocity.0.set_x(0.0);
        velocity.0.set_y(0.0);
        transform.translation.set_x(player::INITIAL_POSITION_Y);
        transform.translation.set_y(player::INITIAL_POSITION_Y);
    }

    match player.movement_state {
        PlayerMovementState::Staying | PlayerMovementState::Running => {
            {
                if input.pressed(KeyCode::Up) || input.pressed(KeyCode::Space) {
                    player.movement_state = PlayerMovementState::Jumping;
                    velocity.0.set_y(player::VELOCITY_ON_JUMP);
                }

                if input.pressed(KeyCode::Left) {
                    velocity.0.set_x(-player::MOVEMENT_VELOCITY);
                }

                if input.pressed(KeyCode::Right) {
                    velocity.0.set_x(player::MOVEMENT_VELOCITY);
                }
            }
        },
        _ => {}
    }
}

fn input_on_paused_game(
    input: &Res<Input<KeyCode>>,
    mut game: &mut ResMut<Game>,
) {
    if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::P) {
        game.state = GameState::Running;
    }
}
