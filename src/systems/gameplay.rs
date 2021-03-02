use crate::effects::{ActiveEffects, VisualEffects};
use crate::game::{Game, GameState};
use bevy::prelude::*;
use crate::world::Velocity;
use crate::player::Player;
use rand::Rng;
use crate::enemies::Enemy;

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

pub fn random_enemy_jump(
    game: Res<Game>,
    mut query: Query<(&mut Velocity), With<(Enemy)>>
) {
    if game.state != GameState::Running {
        return;
    }

    let mut rng = rand::thread_rng();

    // TODO: Make it smarter.
    for mut velocity in query.iter_mut() {
        if velocity.0.y == 0.0 && rng.gen_bool(0.01) {
            velocity.0.y = crate::player::VELOCITY_ON_JUMP * 1.5;
            break;
        }
    }
}
