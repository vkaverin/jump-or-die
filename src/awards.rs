use bevy::prelude::*;
use rand::Rng;

use std::time::Duration;

#[derive(Copy, Clone)]
pub enum Award {
    Score(f32),
    Health(u8),
}

pub struct AwardTimer {
    pub timer: Timer,
    min_time: f32,
    max_time: f32,
}

impl AwardTimer {
    pub fn new(min_time: f32, max_time: f32) -> Self {
        let mut timer = Self {
            timer: Timer::new(Duration::from_secs_f32(0.0), true),
            min_time,
            max_time,
        };
        timer.refill();
        timer
    }

    pub fn refill(&mut self) {
        let mut rng = rand::thread_rng();
        self.timer.set_duration(Duration::from_secs_f32(
            rng.gen_range(self.min_time, self.max_time),
        ));
    }
}
