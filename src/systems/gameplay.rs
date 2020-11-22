use bevy::prelude::*;
use crate::game::{Game, GameState};
use crate::effects::ActiveEffects;

pub fn cleanup_effects(
    game: Res<Game>,
    time: Res<Time>,
    mut query: Query<&mut ActiveEffects>
) {
    if game.state != GameState::Running {
        return;
    }

    for mut effects in query.iter_mut() {
        for effect in &mut effects.effects {
            effect.consume_time(time.delta_seconds);
        }
        effects.effects.retain(|effect| effect.is_active())
    }
}
