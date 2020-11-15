use bevy::prelude::*;
use crate::game::{Game, GameStateLabel, GameState};

pub fn game_state_screen(
    game: Res<Game>,
    mut query: Query<(&mut Text, &mut Draw), With<GameStateLabel>>
) {
    for (mut text, mut draw) in query.iter_mut() {
        match game.state {
            GameState::WaitingForStart => {
                draw.is_visible = true;
                (*text).value = "Press Space to start".to_string();
            }
            GameState::Running => {
                draw.is_visible = false;
                (*text).value = "".to_string();
            }
            GameState::Paused => {
                draw.is_visible = true;
                (*text).value = "Paused".to_string();
            }
            GameState::GameOver => {
                draw.is_visible = true;
                (*text).value = format!("Game over!\nYour score: {}\nPress R to restart", game.score);
            }
        }
    }
}
