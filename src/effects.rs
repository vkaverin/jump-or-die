use bevy::core::Timer;
use bevy::prelude::Transform;
use bevy::render::draw::Draw;
use bevy::sprite::ColorMaterial;

use bevy::asset::{Assets, Handle};
use std::time::Duration;

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
            3.0,
        )
    }

    pub fn new_temporary(name: String, effect: EffectType, time: f32) -> Self {
        Self {
            name,
            effect,
            length: EffectLength::Temporary(time),
        }
    }

    pub fn is_active(&self) -> bool {
        match self.length {
            EffectLength::Permanent => true,
            EffectLength::Temporary(remaining_time) => remaining_time > 0.0,
            EffectLength::Countable(count_left) => count_left > 0,
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

pub struct VisualEffects {
    pub effects: Vec<Box<dyn VisualEffect + Send + Sync>>,
}

impl VisualEffects {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }
}

pub trait VisualEffect {
    fn tick(&mut self, time: f32);
    fn apply(
        &self,
        draw: &mut Draw,
        transform: &mut Transform,
        materials: &mut Assets<ColorMaterial>,
        material: &Handle<ColorMaterial>,
    );
    fn is_expired(&self) -> bool;
}

pub struct PeriodicInvisibility {
    local_timer: Timer,
    global_timer: Timer,
}

impl PeriodicInvisibility {
    pub fn new(period_time: f32, total_time: f32) -> Self {
        Self {
            local_timer: Timer::new(Duration::from_secs_f32(period_time), true),
            global_timer: Timer::new(Duration::from_secs_f32(total_time), false),
        }
    }
}

impl VisualEffect for PeriodicInvisibility {
    fn tick(&mut self, time: f32) {
        self.local_timer.tick(time);
        self.global_timer.tick(time);
    }

    fn apply(
        &self,
        draw: &mut Draw,
        _transform: &mut Transform,
        _materials: &mut Assets<ColorMaterial>,
        _material_handle: &Handle<ColorMaterial>,
    ) {
        if self.global_timer.finished() {
            draw.is_visible = true;
        } else if self.local_timer.just_finished() {
            draw.is_visible = !draw.is_visible;
        }
    }

    fn is_expired(&self) -> bool {
        self.global_timer.finished()
    }
}
