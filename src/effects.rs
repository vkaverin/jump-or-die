pub struct ActiveEffects {
    pub effects: Vec<Effect>,
}

pub struct Effect {
    pub name: String,
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

impl ActiveEffects {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }
}

impl Effect {

    pub fn new_invulnerability() -> Self {
        Self::new_temporary(
            String::from("Invulnerability"),
            EffectType::Invulnerable,
            3.0
        )
    }

    pub fn new_temporary(name: String, effect: EffectType, time: f32) -> Self {
        Self {
            name,
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
