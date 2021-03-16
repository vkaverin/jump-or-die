use crate::game::GameState;
use crate::player::{self, Player, PlayerEvent};
use crate::world::{Collider, Gravity, Velocity};
use bevy::prelude::*;
use bevy::sprite::collide_aabb;

pub fn gravity(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut query: Query<(&mut Velocity, &Sprite, &Transform)>,
) {
    for (mut velocity, sprite, transform) in query.iter_mut() {
        if transform.translation.y > sprite.size.y / 2.0 {
            velocity.add_vertical(-gravity.0 * time.delta_seconds());
        }
    }
}

pub fn movement(
    time: Res<Time>,
    game_window: Res<WindowDescriptor>,
    mut player_entity_query: Query<(Entity, &mut Player)>,
    mut query: Query<(Entity, &mut Velocity, &Sprite, &mut Transform)>,
) {
    let window_half_x = game_window.width as f32 / 2.0;
    let window_left_border = -window_half_x;
    let window_right_border = window_half_x;

    for (entity, mut velocity, sprite, mut transform) in query.iter_mut() {
        {
            let v = velocity.current();
            transform.translation.x += v.x * time.delta_seconds();
            transform.translation.y += v.y * time.delta_seconds();
        }

        if transform.translation.y <= sprite.size.y / 2.0 {
            velocity.drop_vertical();
            transform.translation.y = sprite.size.y / 2.0;
        }

        for (player_entity, mut player) in player_entity_query.iter_mut() {
            if entity == player_entity {
                let player_sprite_half_x = sprite.size.x / 2.0;

                if transform.translation.x - player_sprite_half_x <= window_left_border {
                    transform.translation.x = window_left_border + player_sprite_half_x;
                    velocity.drop_horizontal();
                } else if transform.translation.x + player_sprite_half_x >= window_right_border {
                    transform.translation.x = window_right_border - player_sprite_half_x;
                    velocity.drop_horizontal();
                }
                player::update_movement_state(&mut player, &velocity);
            }
        }
    }
}

pub fn collisions(
    mut commands: Commands,
    mut events: EventWriter<PlayerEvent>,
    player_query: Query<(&Player, &Sprite, &Transform)>,
    colliders: Query<(Entity, &Collider, &Sprite, &Transform)>,
) {
    for (_player, player_sprite, player_transform) in player_query.iter() {
        for (collider_entity, collider, collider_sprite, collider_transform) in colliders.iter() {
            let collision = collide_aabb::collide(
                player_transform.translation,
                player_sprite.size,
                collider_transform.translation,
                collider_sprite.size,
            );
            if collision.is_some() {
                match collider {
                    Collider::Solid => {
                        events.send(PlayerEvent::Hit);
                    }
                    Collider::Award(award) => {
                        events.send(PlayerEvent::Award(*award));
                        commands.despawn(collider_entity);
                    }
                }
            }
        }
    }
}
