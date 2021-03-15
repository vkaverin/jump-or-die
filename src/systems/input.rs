use crate::game::{GameState, GameStateEvent, GameStage};
use crate::player::{self, Player, PlayerMovementState};
use crate::world::{Velocity};
use bevy::prelude::*;
use crate::effects::{EntityEffects, EntityEffectDescriptor, SpeedBoost};
use std::collections::{HashMap};
use std::time::Duration;
use crate::systems::debug::DebugBlock;

#[derive(Default)]
struct InputTracker {
    last_press_time: HashMap<KeyCode, f64>,
    last_pressed: Option<KeyCode>,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<InputTracker>()
            .add_stage_after(CoreStage::PreUpdate, GameStage::AcceptInput, StateStage::<GameState>::default())
            .on_state_update(GameStage::AcceptInput, GameState::StartMenu, start_menu_input.system())
            .on_state_update(GameStage::AcceptInput, GameState::Running, running_game_input.system())
            .on_state_update(GameStage::AcceptInput, GameState::Paused, paused_game_input.system())
            .on_state_update(GameStage::AcceptInput, GameState::GameOver, game_over_menu_input.system());
    }
}

fn start_menu_input(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    if input.pressed(KeyCode::Space) {
        state.set_next(GameState::Running).unwrap();
    }
}

fn running_game_input(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut input_tracker: ResMut<InputTracker>,
    mut query: Query<(&mut Player, &mut Velocity, &mut EntityEffects)>,
    #[cfg(feature = "debug")]
    mut debug_query: Query<(Entity, &Children), With<DebugBlock>>,
) {
    if input.just_pressed(KeyCode::D) {
        #[cfg(feature = "debug")]
            {
                for (entity, children) in debug_query.iter_mut() {
                    if let Ok(mut block_visibility) = visibility_query.get_mut(entity) {
                        let visibility = block_visibility.is_visible;
                        block_visibility.is_visible = !visibility;
                        for child in children.iter() {
                            if let Ok(mut text_visibility) = visibility_query.get_mut(*child) {
                                text_visibility.is_visible = !visibility;
                            }
                        }
                    }
                }
            }
    }

    if input.just_pressed(KeyCode::R) {
        state.set_next(GameState::Starting);
        return;
    }

    if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::P) {
        state.set_next(GameState::Paused).unwrap();
        return;
    }

    for (mut player, mut velocity, mut effects) in query.iter_mut() {
        match player.movement_state {
            PlayerMovementState::Staying | PlayerMovementState::Running => {
                if input.pressed(KeyCode::Up) || input.pressed(KeyCode::Space) {
                    player.movement_state = PlayerMovementState::Jumping;
                    velocity.set_vertical(player::VELOCITY_ON_JUMP);
                }

                if input.pressed(KeyCode::Left) {
                    velocity.set_horizontal(-player::MOVEMENT_VELOCITY);
                }

                if input.pressed(KeyCode::Right) {
                    velocity.set_horizontal(player::MOVEMENT_VELOCITY);
                }

                if let Some(just_pressed) = input.get_just_pressed().last() {
                    let now = time.seconds_since_startup();
                    if input_tracker.last_pressed.is_some() && *just_pressed == input_tracker.last_pressed.unwrap() {
                        if let Some(last_press_time) = input_tracker.last_press_time.get(&just_pressed) {
                            if now - last_press_time < 0.5 {
                                let boost = SpeedBoost::horizontal(3.0);
                                let duration = Duration::from_millis(150);
                                effects.active.push(EntityEffectDescriptor::new_temporary(boost, duration))
                            }
                        }
                    }
                    input_tracker.last_press_time.insert(just_pressed.clone(), now);
                    input_tracker.last_pressed = Some(*just_pressed);
                }
            }
            _ => {}
        }
    }
}

fn paused_game_input(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    if input.just_pressed(KeyCode::R) {
        state.set_next(GameState::Starting);
        return;
    }

    if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::P) {
        state.set_next(GameState::Running).unwrap();
        return;
    }
}

fn game_over_menu_input(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    if input.just_pressed(KeyCode::R) {
        state.set_next(GameState::Starting);
        return;
    }
}
