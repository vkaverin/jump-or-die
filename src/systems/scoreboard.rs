use bevy::prelude::*;
use crate::game::{Game, Scoreboard};

pub fn scoreboard(
    game: Res<Game>,
    mut query: Query<&mut Text, With<Scoreboard>>
) {
    for mut text in query.iter_mut() {
        (*text).value = format!("Score: {}. Best score: {}", game.score, game.best_score);
    }
}
