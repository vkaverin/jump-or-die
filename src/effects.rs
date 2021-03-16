use bevy::core::Timer;
use bevy::prelude::{Entity, Transform, Visible};
use bevy::sprite::ColorMaterial;

use crate::world::Velocity;
use bevy::asset::{Assets, Handle};
use bevy::math::Vec2;
use std::time::Duration;

type BoxedEntityEffect = Box<dyn EntityEffect + Send + Sync>;

pub enum EffectDuration {
    Permanent,
    Temporary(Duration),
}

pub trait EntityEffect: Send + Sync {
    fn apply(&mut self, entity: Entity, velocity: &mut Velocity, transform: &mut Transform);
    fn undo(&mut self, entity: Entity, velocity: &mut Velocity, transform: &mut Transform);
}

pub struct EntityEffectDescriptor {
    effect: BoxedEntityEffect,
    duration: EffectDuration,
}

impl EntityEffectDescriptor {
    pub fn new_permanent<T: EntityEffect + 'static>(effect: T) -> Self {
        Self {
            effect: Box::new(effect),
            duration: EffectDuration::Permanent,
        }
    }

    pub fn new_temporary<T: EntityEffect + 'static>(effect: T, duration: Duration) -> Self {
        Self {
            effect: Box::new(effect),
            duration: EffectDuration::Temporary(duration),
        }
    }

    pub fn effect(&mut self) -> &mut BoxedEntityEffect {
        &mut self.effect
    }

    pub fn tick(&mut self, delta: Duration) {
        if let EffectDuration::Temporary(ref mut duration_left) = self.duration {
            if delta >= *duration_left {
                *duration_left = Duration::from_secs(0);
            } else {
                *duration_left -= delta;
            }
        }
    }

    pub fn is_expired(&self) -> bool {
        match self.duration {
            EffectDuration::Permanent => false,
            EffectDuration::Temporary(duration) => duration == Duration::from_secs(0),
        }
    }
}

#[derive(Default)]
pub struct EntityEffects {
    pub active: Vec<EntityEffectDescriptor>,
}

pub struct SpeedBoost {
    pub boost: Vec2,
}

impl SpeedBoost {
    pub fn horizontal(v: f32) -> Self {
        Self {
            boost: Vec2::new(v, 1.0),
        }
    }

    pub fn vertical(v: f32) -> Self {
        Self {
            boost: Vec2::new(1.0, v),
        }
    }
}

impl From<Vec2> for SpeedBoost {
    fn from(v: Vec2) -> Self {
        Self { boost: v }
    }
}

impl EntityEffect for SpeedBoost {
    fn apply(&mut self, entity: Entity, velocity: &mut Velocity, transform: &mut Transform) {
        velocity.set_boost(self.boost);
    }
    fn undo(&mut self, entity: Entity, velocity: &mut Velocity, transform: &mut Transform) {
        velocity.drop_boost();
    }
}

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
        visibility: &mut Visible,
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
        self.local_timer.tick(Duration::from_secs_f32(time));
        self.global_timer.tick(Duration::from_secs_f32(time));
    }

    fn apply(
        &self,
        visibility: &mut Visible,
        _transform: &mut Transform,
        _materials: &mut Assets<ColorMaterial>,
        _material_handle: &Handle<ColorMaterial>,
    ) {
        if self.global_timer.finished() {
            visibility.is_visible = true;
        } else if self.local_timer.just_finished() {
            visibility.is_visible = !visibility.is_visible;
        }
    }

    fn is_expired(&self) -> bool {
        self.global_timer.finished()
    }
}
