use crate::world::{Gravity, Velocity, Collidable};
use bevy::prelude::*;
use crate::player::{self, Player, PlayerMovementState, PlayerEvent};
use crate::game::{Game, GameState};
use bevy::sprite::collide_aabb::{self, Collision};

pub fn gravity(
    time: Res<Time>,
    game: Res<Game>,
    gravity: Res<Gravity>,
    mut query: Query<(&mut Velocity, &Sprite, &mut Transform)>,
) {
    if game.state != GameState::Running {
        return;
    }

    for (mut velocity, sprite,  mut transform) in query.iter_mut() {
        if transform.translation.y() > sprite.size.y() / 2.0 {
            *velocity.0.y_mut() -= gravity.0 * time.delta_seconds;
        }
    }
}

pub fn movement(
    time: Res<Time>,
    game: Res<Game>,
    mut player_entity_query: Query<(Entity, &mut Player)>,
    mut query: Query<(Entity, &mut Velocity, &Sprite, &mut Transform)>
) {
    if game.state != GameState::Running {
        return;
    }

    for (entity, mut velocity, sprite, mut transform) in query.iter_mut() {
        *transform.translation.x_mut() += velocity.0.x() * time.delta_seconds;
        *transform.translation.y_mut() += velocity.0.y() * time.delta_seconds;

        if transform.translation.y() <= sprite.size.y() / 2.0 {
            velocity.0.set_y(0.0);
            transform.translation.set_y(sprite.size.y() / 2.0);
        }

        for (player_entity, mut player) in player_entity_query.iter_mut() {
            if entity == player_entity {
                player::update_movement_state(&mut player, &velocity);
            }
        }
    }
}

pub fn collisions(
    mut events: ResMut<Events<PlayerEvent>>,
    player_query: Query<(&Player, &Sprite, &Transform)>,
    collidables: Query<(&Collidable, &Sprite, &Transform)>,
) {

    for (_player, player_sprite, player_transform) in player_query.iter() {
        for (_collidable, collidable_sprite, collidable_transform) in collidables.iter() {
            if let Some(collision) = collide_aabb::collide(
                player_transform.translation,
                player_sprite.size,
                collidable_transform.translation,
                collidable_sprite.size
            ) {
                events.send(PlayerEvent::Hit);
                break;
            }
        }
    }
}
