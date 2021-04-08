use crate::awards::Award;
use crate::effects::{ActiveEffects, Effect, EffectType, PeriodicInvisibility, VisualEffects};
use crate::game::{Game, GameState};
use crate::player::{Player, PlayerEvent};
use bevy::prelude::*;

pub fn player_events(
    mut game: ResMut<Game>,
    mut state: ResMut<State<GameState>>,
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
                            state.set(GameState::GameOver).unwrap();
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
