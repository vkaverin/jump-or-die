use crate::effects::{ActiveEffects, EntityEffects, VisualEffects};
use crate::enemies::Enemy;
use crate::game::{Game, GameEntity, GameState};
use crate::player::{self, Player};
use crate::world::Velocity;

use bevy::prelude::*;
use rand::Rng;

pub fn apply_effects(
    time: Res<Time>,
    mut query: Query<(Entity, &mut EntityEffects, &mut Velocity, &mut Transform)>,
) {
    for (entity, mut effects, mut velocity, mut transform) in query.iter_mut() {
        for effect in &mut effects.active {
            effect.tick(time.delta());
            if !effect.is_expired() {
                effect.effect().apply(entity, &mut velocity, &mut transform);
            } else {
                effect.effect().undo(entity, &mut velocity, &mut transform);
            }
        }
    }
}

pub fn cleanup_effects(
    time: Res<Time>,
    mut query: Query<(&mut ActiveEffects, &mut VisualEffects)>,
) {
    for (mut effects, mut visual_effects) in query.iter_mut() {
        for effect in &mut effects.effects {
            effect.consume_time(time.delta_seconds());
        }
        effects.effects.retain(|effect| effect.is_active());
        visual_effects.effects.retain(|effect| !effect.is_expired());
    }
}

pub fn random_enemy_jump(mut query: Query<&mut Velocity, With<Enemy>>) {
    let mut rng = rand::thread_rng();

    // TODO: Make it smarter.
    for mut velocity in query.iter_mut() {
        let mut v = velocity.current();
        if v.y == 0.0 && rng.gen_bool(0.01) {
            velocity.set_vertical(crate::player::VELOCITY_ON_JUMP * 1.25);
            break;
        }
    }
}

pub fn start_game(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut state: ResMut<State<GameState>>,
    mut player_query: Query<(
        &mut Player,
        &mut EntityEffects,
        &mut ActiveEffects,
        &mut VisualEffects,
        &mut Velocity,
        &mut Visible,
        &mut Transform,
    )>,
    entities: Query<Entity, (With<GameEntity>, Without<Player>)>,
) {
    game.score = 0.0;

    for entity in entities.iter() {
        commands.despawn(entity);
    }

    for (
        mut player,
        mut entity_effects,
        mut active_effects,
        mut visual_effects,
        mut velocity,
        mut visibility,
        mut transform,
    ) in player_query.iter_mut()
    {
        player.health = player.max_health;

        entity_effects.active.clear();
        active_effects.effects.clear();
        visual_effects.effects.clear();
        velocity.reset();
        visibility.is_visible = true;
        transform.translation.x = player::INITIAL_POSITION_X;
        transform.translation.y = player::INITIAL_POSITION_Y;
    }

    state.set_next(GameState::Running).unwrap();
}
