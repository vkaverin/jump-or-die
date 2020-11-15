use crate::enemies::Enemy;
use crate::game::{Game, GameState, GameStateEvent};
use crate::player;
use crate::player::{Player, PlayerEvent};
use crate::world::Velocity;
use bevy::prelude::*;
use crate::effects::{Effects, Effect, EffectType};

pub fn player_events(
    commands: &mut Commands,
    mut game: ResMut<Game>,
    mut event_reader: Local<EventReader<PlayerEvent>>,
    events: Res<Events<PlayerEvent>>,
    mut player_query: Query<(Entity, &mut Player, &mut Effects)>
) {
    for e in event_reader.iter(&events) {
        match e {
            PlayerEvent::Hit => {
                for (entity, mut player, mut effects) in player_query.iter_mut() {
                    let is_invulnerable = {
                        let mut is_invulnerable = false;
                        for effect in &effects.effects {
                            if effect.effect_type == EffectType::Invulnerable && !effect.is_expired() {
                                is_invulnerable = true;
                            }
                        }
                        is_invulnerable
                    };

                    if !is_invulnerable && player.health > 0 {
                        player.health -= 1;
                        if player.health == 0 {
                            game.state = GameState::GameOver;
                        } else {
                            commands.insert_one(entity, effects.effects.push(Effect::new_temporary(EffectType::Invulnerable, 3.0)));
                        }
                    }
                }
            }
        }
    }
}

pub fn game_state_events(
    commands: &mut Commands,
    events: Res<Events<GameStateEvent>>,
    mut event_reader: Local<EventReader<GameStateEvent>>,
    mut game: ResMut<Game>,
    mut player_query: Query<(&mut Player, &mut Velocity, &mut Transform)>,
    enemies_query: Query<Entity, With<Enemy>>,
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
    player_query: &mut Query<(&mut Player, &mut Velocity, &mut Transform)>,
    enemies_query: &Query<Entity, With<Enemy>>,
) {
    game.state = GameState::Running;
    game.score = 0.0;
    for (mut player, mut velocity, mut transform) in player_query.iter_mut() {
        player.health = player.max_health;
        velocity.0 = Vec2::zero();
        transform.translation.set_x(player::INITIAL_POSITION_X);
        transform.translation.set_y(player::INITIAL_POSITION_Y);
    }

    for enemy_entity in enemies_query.iter() {
        commands.despawn(enemy_entity);
    }
}
