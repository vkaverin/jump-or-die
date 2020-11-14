use crate::enemies::Award;

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
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::WaitingForStart,
            score: 0.0,
        }
    }

    pub fn is_running(&self) -> bool {
        self.state == GameState::Running
    }

    pub fn consume_award(&mut self, award: &Award) {
        match award {
            Award::Score(score) => {
                self.score += score;
            }
        }
    }
}

pub struct Scoreboard;
