use bevy::prelude::*;
use crate::game::Game;
use crate::enemies::{Enemy, GivesAward};
use crate::player::Player;

pub fn collect_enemy_awards(
    mut game: ResMut<Game>,
    player_query: Query<(&Sprite, &Transform), With<Player>>,
    mut query: Query<(&mut GivesAward, &Sprite, &Transform), With<Enemy>>
) {
    for (player_sprite, player_transform) in player_query.iter() {
        for (mut award, enemy_sprite, enemy_transform, ) in query.iter_mut() {
            if award.already_taken {
                continue;
            }
            if enemy_transform.translation.x() + enemy_sprite.size.x() / 2.0 < player_transform.translation.x() - player_sprite.size.x() / 2.0 {
                award.take();
                game.consume_award(&award.award)
            }
        }
    }
}
