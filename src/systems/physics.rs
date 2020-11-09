use crate::world::{Gravity, Velocity};
use bevy::prelude::*;
use crate::player::{Player, PlayerMovementState};

pub fn gravity(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut query: Query<(&mut Velocity, &mut Transform)>,
) {
    for (mut velocity, mut _transform) in query.iter_mut() {
        let y = (velocity.0.y() - gravity.0 * time.delta_seconds).max(-gravity.0);
        velocity.0.set_y(y);
    }
}

pub fn movement(
    time: Res<Time>,
    mut player_entity_query: Query<(Entity, &mut Player)>,
    mut query: Query<(Entity, &mut Velocity, &Sprite, &mut Transform)>
) {
    for (entity, mut velocity, sprite, mut transform) in query.iter_mut() {
        *transform.translation.x_mut() += velocity.0.x() * time.delta_seconds;
        *transform.translation.y_mut() += velocity.0.y() * time.delta_seconds;
        if transform.translation.y() < sprite.size.y() / 2.0 {
            velocity.0.set_y(0.0);
            transform.translation.set_y(sprite.size.y() / 2.0);

            for (player_entity, mut player) in player_entity_query.iter_mut() {
                if entity == player_entity {
                    player.movement_state = PlayerMovementState::Staying;
                }
            }
        }
    }
}

// fn collisions(
//     time: Res<Time>,
//     friction: Res<Friction>,
//     mut query: Query<(&mut Velocity, &Sprite, &mut Transform)>,
//     collidables: Query<(&Collidable, &Sprite, &Transform)>
// ) {
//     for (mut velocity, sprite, mut transform) in query.iter_mut() {
//         for (_collidable, colliable_sprite, collidable_transform) in collidables.iter() {
//             if let Some(collision) = collide(transform.translation, sprite.size, collidable_transform.translation, colliable_sprite.size) {
//                 match collision {
//                     Collision::Top | Collision::Bottom  => {
//                         velocity.0.set_y(0.0);
//                         let sign = match collision {
//                             Collision::Top => 1.0,
//                             Collision::Bottom => -1.0,
//                             _ => {
//                                 panic!("never happens");
//                             }
//                         };
//                         transform.translation.set_y(collidable_transform.translation.y() + sign * (colliable_sprite.size.y() / 2.0 + sprite.size.y() / 2.0));
//                         let new_x = velocity.0.x().signum() * (velocity.0.x().abs() - friction.0 * time.delta_seconds);
//
//                         if velocity.0.x().signum() != new_x.signum() {
//                             velocity.0.set_x(0.0);
//                         } else {
//                             velocity.0.set_x(new_x);
//                         }
//                     },
//                     Collision::Left | Collision::Right  => {
//                         velocity.0.set_x(0.0);
//                         let sign = match collision {
//                             Collision::Right => 1.0,
//                             Collision::Left => -1.0,
//                             _ => {
//                                 panic!("never happens");
//                             }
//                         };
//                         transform.translation.set_x(collidable_transform.translation.x() + sign * (colliable_sprite.size.x() / 2.0 + sprite.size.x() / 2.0));
//                         let new_y = velocity.0.y().signum() * (velocity.0.y().abs() - friction.0 * time.delta_seconds);
//
//                         if velocity.0.y().signum() != new_y.signum() {
//                             velocity.0.set_y(0.0);
//                         } else {
//                             velocity.0.set_y(new_y);
//                         }
//                     },
//                 }
//             }
//         }
//     }
// }
