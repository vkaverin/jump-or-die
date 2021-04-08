use crate::effects::{EntityEffectDescriptor, EntityEffects, SpeedBoost};
use crate::game::GameState;
use crate::player::{self, Player, PlayerMovementState};
use crate::world::Velocity;
use bevy::prelude::*;
use std::collections::HashMap;
use std::time::{Duration};

#[cfg(feature = "debug")]
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
            .add_system_set(
                SystemSet::on_update(GameState::StartMenu)
                    .with_system(start_menu_input.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(running_game_input.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Paused)
                    .with_system(paused_game_input.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::GameOver)
                    .with_system(game_over_menu_input.system()),
            );
    }
}

fn start_menu_input(mut input: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if input.pressed(KeyCode::Space) {
        input.reset(KeyCode::Space);
        state.set(GameState::Starting).unwrap();
    }
}

fn running_game_input(
    time: Res<Time>,
    mut input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut input_tracker: ResMut<InputTracker>,
    mut query: Query<(&mut Player, &mut Velocity, &mut EntityEffects)>,
    #[cfg(feature = "debug")] mut debug_query: Query<(Entity, &Children), With<DebugBlock>>,
    #[cfg(feature = "debug")] mut visibility_query: Query<(&mut Visible), With<DebugBlock>>,
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
        input.reset(KeyCode::R);
        state.set(GameState::Starting).unwrap();
    } else if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::P) {
        input.reset(KeyCode::P);
        input.reset(KeyCode::Escape);
        state.set(GameState::Paused).unwrap();
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
                    if input_tracker.last_pressed.is_some()
                        && *just_pressed == input_tracker.last_pressed.unwrap()
                    {
                        if let Some(last_press_time) =
                            input_tracker.last_press_time.get(&just_pressed)
                        {
                            if now - last_press_time < 0.5 {
                                let boost = SpeedBoost::horizontal(3.0);
                                let duration = Duration::from_millis(150);
                                effects
                                    .active
                                    .push(EntityEffectDescriptor::new_temporary(boost, duration))
                            }
                        }
                    }

                    input_tracker.last_press_time.insert(*just_pressed, now);
                    input_tracker.last_pressed = Some(*just_pressed);
                }
            }
            _ => {}
        }
    }
}

fn paused_game_input(mut input: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if input.just_pressed(KeyCode::R) {
        input.reset(KeyCode::R);
        state.set(GameState::Starting).unwrap();
    } else if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::P) {
        input.reset(KeyCode::P);
        input.reset(KeyCode::Escape);
        state.set(GameState::Running).unwrap();
    }
}

fn game_over_menu_input(mut input: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if input.just_pressed(KeyCode::R) {
        input.reset(KeyCode::R);
        state.set(GameState::Starting).unwrap();
    }
}
