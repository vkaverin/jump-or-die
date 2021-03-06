use bevy::prelude::*;

pub struct GameEntity;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum GameStage {
    AcceptInput,
    Game,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    StartMenu,
    Starting,
    Running,
    Paused,
    GameOver,
}

impl PartialEq<GameState> for State<GameState> {
    fn eq(&self, other: &GameState) -> bool {
        *self.current() == *other
    }
}

#[derive(Debug, Default)]
pub struct Game {
    pub score: f32,
    pub best_score: f32,
}
