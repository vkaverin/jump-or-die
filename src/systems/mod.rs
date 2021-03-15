pub mod debug;
pub mod events;
pub mod gameplay;
pub mod hud;
pub mod input;
pub mod physics;
pub mod spawning;
pub mod visual_effects;

pub mod plugins {
    pub use super::debug::DebugPlugin;
    pub use super::hud::HudPlugin;
    pub use super::input::InputPlugin;
}
