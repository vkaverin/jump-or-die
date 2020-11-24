use bevy::prelude::*;
use crate::effects::VisualEffects;
use crate::player::Player;
use crate::game::{Game, GameState};

pub fn run_visual_effects(
    time: Res<Time>,
    game: Res<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut VisualEffects, &mut Draw, &mut Transform, &Handle<ColorMaterial>), With<Player>>
) {
    if game.state != GameState::Running {
        return;
    }

    for (mut visual_effects, mut draw, mut transform, material) in query.iter_mut() {
        for effect in &mut visual_effects.effects {
            effect.tick(time.delta_seconds);
            effect.apply(&mut draw, &mut transform, &mut materials, &material)
        }
    }
}
