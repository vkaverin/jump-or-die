use crate::awards::Award;

pub enum GameStateEvent {
    Restart,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    WaitingForStart,
    Running,
    Paused,
    GameOver,
}

#[derive(Debug)]
pub struct Game {
    pub state: GameState,
    pub score: f32,
    pub best_score: f32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::WaitingForStart,
            score: 0.0,
            best_score: 0.0,
        }
    }

    pub fn is_running(&self) -> bool {
        self.state == GameState::Running
    }
}
