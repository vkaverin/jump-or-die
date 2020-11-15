pub struct Effects {
    pub effects: Vec<Effect>,
}

pub struct Effect {
    is_temporary: bool,
    remaining_time: f32,
    pub effect_type: EffectType,
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
    pub fn new_temporary(effect_type: EffectType, time: f32) -> Self {
        Self {
            effect_type,
            is_temporary: true,
            remaining_time: time,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.is_temporary && self.remaining_time == 0.0
    }

    pub fn consume_time(&mut self, time_delta: f32) {
        if self.is_temporary && self.remaining_time > 0.0 {
            self.remaining_time = (self.remaining_time - time_delta).max(0.0);
        }
    }
}
