use crate::game::{Game, GameState, GameStateEvent};
use crate::player::{self, Player, PlayerMovementState};
use crate::world::Velocity;
use bevy::prelude::*;

pub fn input(
    input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut game_events: ResMut<Events<GameStateEvent>>,
    mut query: Query<(&mut Player, &mut Velocity, &mut Transform)>
) {
    for (mut player, mut velocity, mut transform) in query.iter_mut() {
        match game.state {
            GameState::WaitingForStart => {
                if input.pressed(KeyCode::Space) {
                    game.state = GameState::Running;
                }
            }
            GameState::Running => {
                input_on_running_game(&input, &mut game_events, &mut game, &mut player, &mut velocity);
            }
            GameState::Paused => {
                input_on_paused_game(&input, &mut game_events, &mut game);
            }
            GameState::GameOver => {
                input_on_game_over(&input, &mut game_events);
            }
        }
    }
}

fn input_on_running_game(
    input: &Res<Input<KeyCode>>,
    mut game_events: &mut ResMut<Events<GameStateEvent>>,
    mut game: &mut ResMut<Game>,
    mut player: &mut Mut<Player>,
    mut velocity: &mut Mut<Velocity>,
) {
    if input.just_pressed(KeyCode::R) {
        game_events.send(GameStateEvent::Restart);
        return;
    }

    if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::P){
        game.state = GameState::Paused;
        return;
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
    mut game_events: &mut ResMut<Events<GameStateEvent>>,
    mut game: &mut ResMut<Game>,
) {
    if input.just_pressed(KeyCode::R) {
        game_events.send(GameStateEvent::Restart);
        return;
    }

    if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::P) {
        game.state = GameState::Running;
    }
}

fn input_on_game_over(
    input: &Res<Input<KeyCode>>,
    mut game_events: &mut ResMut<Events<GameStateEvent>>,
) {
    if input.just_pressed(KeyCode::R) {
        game_events.send(GameStateEvent::Restart);
        return;
    }
}
