pub struct Effects {
    pub effects: Vec<Effect>,
}

pub struct Effect {
    pub length: EffectLength,
    pub effect: EffectType,
}

pub enum EffectLength {
    Permanent,
    Temporary(f32),
    Countable(u8),
}

#[derive(PartialEq, Eq)]
pub enum EffectType {
    Invulnerable,
}

impl Effects {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }
}

impl Effect {

    pub fn new_temporary(effect: EffectType, time: f32) -> Self {
        Self {
            effect,
            length: EffectLength::Temporary(time)
        }
    }

    pub fn is_active(&self) -> bool {
        match self.length {
            EffectLength::Permanent => {
                true
            }
            EffectLength::Temporary(remaining_time) => {
                remaining_time > 0.0
            }
            EffectLength::Countable(count_left) => {
                count_left > 0
            }
        }
    }

    pub fn consume_time(&mut self, time_delta: f32) {
        match self.length {
            EffectLength::Temporary(time_left) => {
                let time_left = (time_left - time_delta).max(0.0);
                self.length = EffectLength::Temporary(time_left);
            }
            _ => {}
        }
    }
}
