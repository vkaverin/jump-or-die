use crate::game::{Game, GameState, GameStateEvent};
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
            .add_system(input.system());
    }
}

fn input(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut input_tracker: ResMut<InputTracker>,
    mut game: ResMut<Game>,
    mut game_events: ResMut<Events<GameStateEvent>>,
    mut query: Query<(&mut Player, &mut Velocity, &mut EntityEffects)>,
    mut visibility_query: Query<&mut Visible>,
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

    for (mut player, mut velocity, mut effects) in query.iter_mut() {
        match game.state {
            GameState::WaitingForStart => {
                if input.pressed(KeyCode::Space) {
                    game.state = GameState::Running;
                }
            }
            GameState::Running => {
                input_on_running_game(
                    &time,
                    &input,
                    &mut game_events,
                    &mut game,
                    &mut input_tracker,
                    &mut player,
                    &mut velocity,
                    &mut effects,
                );
            }
            GameState::Paused => {
                input_on_paused_game(&input, &mut game_events, &mut game);
            }
            GameState::GameOver => {
                input_on_game_over(&input, &mut game_events);
            }
        }
    }
}

fn input_on_running_game(
    time: &Res<Time>,
    input: &Res<Input<KeyCode>>,
    game_events: &mut ResMut<Events<GameStateEvent>>,
    game: &mut ResMut<Game>,
    input_tracker: &mut ResMut<InputTracker>,
    player: &mut Mut<Player>,
    velocity: &mut Mut<Velocity>,
    effects: &mut Mut<EntityEffects>,
) {
    if input.just_pressed(KeyCode::R) {
        game_events.send(GameStateEvent::Restart);
        return;
    }

    if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::P) {
        game.state = GameState::Paused;
        return;
    }

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

fn input_on_paused_game(
    input: &Res<Input<KeyCode>>,
    game_events: &mut ResMut<Events<GameStateEvent>>,
    game: &mut ResMut<Game>,
) {
    if input.just_pressed(KeyCode::R) {
        game_events.send(GameStateEvent::Restart);
        return;
    }

    if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::P) {
        game.state = GameState::Running;
    }
}

fn input_on_game_over(
    input: &Res<Input<KeyCode>>,
    game_events: &mut ResMut<Events<GameStateEvent>>,
) {
    if input.just_pressed(KeyCode::R) {
        game_events.send(GameStateEvent::Restart);
        return;
    }
}
