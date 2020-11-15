use bevy::prelude::*;
use crate::game::{GameState, Game, GameStateEvent};
use crate::player::{PlayerEvent, Player};
use crate::world::Velocity;
use crate::enemies::Enemy;
use crate::player;

pub fn player_events(
    mut game: ResMut<Game>,
    mut event_reader: Local<EventReader<PlayerEvent>>,
    events: Res<Events<PlayerEvent>>
) {
    for e in event_reader.iter(&events) {
        match e {
            PlayerEvent::Hit => {
                game.state = GameState::GameOver;
            }
        }
    }
}

pub fn game_state_events(
    commands: &mut Commands,
    events: Res<Events<GameStateEvent>>,
    mut event_reader: Local<EventReader<GameStateEvent>>,
    mut game: ResMut<Game>,
    mut player_query: Query<(&mut Velocity, &mut Transform), With<Player>>,
    enemies_query: Query<Entity, With<Enemy>>
) {
    for e in event_reader.iter(&events) {
        match e {
            GameStateEvent::Restart => {
                restart_game(commands, &mut game, &mut player_query, &enemies_query);
            }
        }
    }
}

fn restart_game(
    commands: &mut Commands,
    game: &mut ResMut<Game>,
    player_query: &mut Query<(&mut Velocity, &mut Transform), With<Player>>,
    enemies_query: &Query<Entity, With<Enemy>>
) {
    game.state = GameState::Running;
    game.score = 0.0;
    for (mut velocity, mut transform) in player_query.iter_mut() {
        velocity.0 = Vec2::zero();
        transform.translation.set_x(player::INITIAL_POSITION_X);
        transform.translation.set_y(player::INITIAL_POSITION_Y);
    }

    for enemy_entity in enemies_query.iter() {
        commands.despawn(enemy_entity);
    }
}
