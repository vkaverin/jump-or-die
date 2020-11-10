use crate::world::{Gravity, Velocity, Collidable};
use bevy::prelude::*;
use crate::player::{Player, PlayerMovementState};
use crate::game::{Game, GameState};
use bevy::sprite::collide_aabb::{collide, Collision};

pub fn gravity(
    time: Res<Time>,
    game: Res<Game>,
    gravity: Res<Gravity>,
    mut query: Query<(&mut Velocity, &mut Transform)>,
) {
    if game.state != GameState::Running {
        return;
    }

    for (mut velocity, mut _transform) in query.iter_mut() {
        let y = (velocity.0.y() - gravity.0 * time.delta_seconds).max(-gravity.0);
        velocity.0.set_y(y);
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

        for (player_entity, mut player) in player_entity_query.iter_mut() {
            if entity == player_entity && velocity.0 == Vec2::zero() {
                player.movement_state = PlayerMovementState::Staying;
            }
        }
    }
}

pub fn collisions(
    mut query: Query<(Entity, &mut Velocity, &Sprite, &mut Transform)>,
    collidables: Query<(Entity, &Collidable, &Sprite, &Transform)>
) {
    for (entity, mut velocity, sprite, mut transform) in query.iter_mut() {
        for (collidable_entity, _collidable, colliable_sprite, collidable_transform) in collidables.iter() {
            if entity == collidable_entity {
                continue;
            }

            let maybe_collision =  collide(
                transform.translation,
                sprite.size,
                collidable_transform.translation,
                colliable_sprite.size
            );
            if let Some(collision) = maybe_collision {
                match collision {
                    Collision::Top => {
                        velocity.0.set_y(0.0);
                        transform.translation.set_y(collidable_transform.translation.y() + colliable_sprite.size.y() / 2.0 + sprite.size.y() / 2.0);
                    },
                    Collision::Bottom => {
                        velocity.0.set_y(0.0);
                        transform.translation.set_y(collidable_transform.translation.y() - colliable_sprite.size.y() / 2.0 - sprite.size.y() / 2.0);
                    },
                    Collision::Left => {
                        velocity.0.set_x(0.0);
                        transform.translation.set_x(collidable_transform.translation.x() - colliable_sprite.size.x() / 2.0 - sprite.size.x() / 2.0);
                    },
                    Collision::Right => {
                        velocity.0.set_x(0.0);
                        transform.translation.set_x(collidable_transform.translation.x() + colliable_sprite.size.x() / 2.0 + sprite.size.x() / 2.0);
                    }
                }
            }
        }
    }
}
