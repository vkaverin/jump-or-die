use crate::awards::Award;
use crate::effects::{ActiveEffects, Effect, EffectType, PeriodicInvisibility, VisualEffects};
use crate::game::{Game, GameState, GameStateEvent};
use crate::player;
use crate::player::{Player, PlayerEvent};
use crate::world::{Collider, Velocity};
use bevy::prelude::*;

pub fn player_events(
    mut game: ResMut<Game>,
    mut event_reader: EventReader<PlayerEvent>,
    mut player_query: Query<(&mut Player, &mut ActiveEffects, &mut VisualEffects)>,
) {
    for e in event_reader.iter() {
        match e {
            PlayerEvent::Hit => {
                for (mut player, mut effects, mut visual_effects) in player_query.iter_mut() {
                    let is_invulnerable = {
                        let mut is_invulnerable = false;
                        for effect in &effects.effects {
                            if effect.effect == EffectType::Invulnerable && effect.is_active() {
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
                            effects.effects.push(Effect::new_invulnerability());
                            visual_effects
                                .effects
                                .push(Box::new(PeriodicInvisibility::new(0.2, 3.0)));
                        }
                    }
                }
            }
            PlayerEvent::Award(award) => match award {
                Award::Score(score) => {
                    game.score += score;
                    game.best_score = game.best_score.max(game.score);
                }
                Award::Health(health) => {
                    for (mut player, mut _effects, mut _visual_effects) in player_query.iter_mut() {
                        player.health = (player.health + health).min(player.max_health);
                    }
                }
            },
        }
    }
}

pub fn game_state_events(
    commands: &mut Commands,
    mut event_reader: EventReader<GameStateEvent>,
    mut game: ResMut<Game>,
    mut player_query: Query<(
        &mut Player,
        &mut ActiveEffects,
        &mut VisualEffects,
        &mut Velocity,
        &mut Visible,
        &mut Transform,
    )>,
    colliders: Query<Entity, With<Collider>>,
) {
    for e in event_reader.iter() {
        match e {
            GameStateEvent::Restart => {
                restart_game(commands, &mut game, &mut player_query, &colliders);
            }
        }
    }
}

fn restart_game(
    commands: &mut Commands,
    game: &mut ResMut<Game>,
    player_query: &mut Query<(
        &mut Player,
        &mut ActiveEffects,
        &mut VisualEffects,
        &mut Velocity,
        &mut Visible,
        &mut Transform,
    )>,
    colliders: &Query<Entity, With<Collider>>,
) {
    game.state = GameState::Running;
    game.score = 0.0;
    for (
        mut player,
        mut active_effects,
        mut visual_effects,
        mut velocity,
        mut visibility,
        mut transform,
    ) in player_query.iter_mut()
    {
        player.health = player.max_health;
        active_effects.effects.clear();
        visual_effects.effects.clear();
        velocity.reset();
        visibility.is_visible = true;
        transform.translation.x = player::INITIAL_POSITION_X;
        transform.translation.y = player::INITIAL_POSITION_Y;
    }

    for entity in colliders.iter() {
        commands.despawn(entity);
    }
}
