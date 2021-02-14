use crate::effects::VisualEffects;
use crate::game::{Game, GameState};
use crate::player::Player;
use bevy::prelude::*;

pub fn run_visual_effects(
    time: Res<Time>,
    game: Res<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<
        (
            &mut VisualEffects,
            &mut Visible,
            &mut Transform,
            &Handle<ColorMaterial>,
        ),
        With<Player>,
    >,
) {
    if game.state != GameState::Running {
        return;
    }

    for (mut visual_effects, mut visibility, mut transform, material) in query.iter_mut() {
        for effect in &mut visual_effects.effects {
            effect.tick(time.delta_seconds());
            effect.apply(&mut visibility, &mut transform, &mut materials, &material)
        }
    }
}
