use crate::game::{Game, GameState, GameStateEvent};
use crate::player::{self, Player, PlayerMovementState};
use crate::systems::debug::DebugBlock;
use crate::world::{Velocity, Deformation};
use bevy::prelude::*;

pub fn input(
    input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut game_events: ResMut<Events<GameStateEvent>>,
    mut query: Query<(&mut Player, &mut Velocity, &mut Deformation)>,
    #[cfg(feature = "debug")]
    mut debug_query: Query<(Entity, &Children), With<DebugBlock>>,
    mut visibility_query: Query<&mut Visible>,
) {
    if input.just_pressed(KeyCode::D) {
        #[cfg(feature = "debug")]
        {
            for (entity, children) in debug_query.iter_mut() {
                if let Ok(mut block_visibility) = visibility_query.get_mut(entity) {
                    let visibility = block_visibility.is_visible;
                    block_visibility.is_visible = !visibility;
                    for child in children.iter() {
                        if let Ok(mut text_visibility) = visibility_query.get_mut(*child) {
                            text_visibility.is_visible = !visibility;
                        }
                    }
                }
            }
        }
    }

    for (mut player, mut velocity, mut deformation) in query.iter_mut() {
        match game.state {
            GameState::WaitingForStart => {
                if input.pressed(KeyCode::Space) {
                    game.state = GameState::Running;
                }
            }
            GameState::Running => {
                input_on_running_game(
                    &input,
                    &mut game_events,
                    &mut game,
                    &mut player,
                    &mut velocity,
                    &mut deformation
                );
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
    game_events: &mut ResMut<Events<GameStateEvent>>,
    game: &mut ResMut<Game>,
    player: &mut Mut<Player>,
    velocity: &mut Mut<Velocity>,
    deformation: &mut Mut<Deformation>,
) {
    if input.just_pressed(KeyCode::R) {
        game_events.send(GameStateEvent::Restart);
        return;
    }

    if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::P) {
        game.state = GameState::Paused;
        return;
    }

    match player.movement_state {
        PlayerMovementState::Staying | PlayerMovementState::Running => {
            if input.pressed(KeyCode::Up) || input.pressed(KeyCode::Space) {
                player.movement_state = PlayerMovementState::Jumping;
                velocity.0.y = player::VELOCITY_ON_JUMP;
            }

            if input.pressed(KeyCode::Left) {
                velocity.0.x = -player::MOVEMENT_VELOCITY;
            }

            if input.pressed(KeyCode::Right) {
                velocity.0.x = player::MOVEMENT_VELOCITY;
            }

            if input.pressed(KeyCode::Down) {
                deformation.top = -5.0;
            } else {
                deformation.top = 5.0;
            }
        }
        _ => {}
    }
}

fn input_on_paused_game(
    input: &Res<Input<KeyCode>>,
    game_events: &mut ResMut<Events<GameStateEvent>>,
    game: &mut ResMut<Game>,
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
    game_events: &mut ResMut<Events<GameStateEvent>>,
) {
    if input.just_pressed(KeyCode::R) {
        game_events.send(GameStateEvent::Restart);
        return;
    }
}
