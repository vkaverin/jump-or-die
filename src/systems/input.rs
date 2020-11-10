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

    if player.movement_state == PlayerMovementState::Staying
        || player.movement_state == PlayerMovementState::Running
    {
        let mut dx = 0.0;
        let mut dy = 0.0;

        if input.pressed(KeyCode::Up) || input.pressed(KeyCode::Space) {
            player.movement_state = PlayerMovementState::Jumping;
            dy = player::VELOCITY_ON_JUMP;
        }

        let new_x = (velocity.0.x() + dx).max(-300.0).min(300.0);
        let new_y = velocity.0.y() + dy;
        velocity.0.set_x(new_x);
        velocity.0.set_y(new_y);
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
