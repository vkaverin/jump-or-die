use crate::effects::{ActiveEffects, VisualEffects};
use crate::game::{Game, GameState};
use bevy::prelude::*;

pub fn cleanup_effects(
    game: Res<Game>,
    time: Res<Time>,
    mut query: Query<(&mut ActiveEffects, &mut VisualEffects)>,
) {
    if game.state != GameState::Running {
        return;
    }

    for (mut effects, mut visual_effects) in query.iter_mut() {
        for effect in &mut effects.effects {
            effect.consume_time(time.delta_seconds());
        }
        effects.effects.retain(|effect| effect.is_active());
        visual_effects.effects.retain(|effect| !effect.is_expired());
    }
}
