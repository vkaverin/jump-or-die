use crate::effects::{ActiveEffects, EffectLength};
use crate::game::{Game, GameState};
use crate::player::Player;
use bevy::prelude::*;

const STARTUP_STAGE: &str = "hud_startup";

const PLAYER_STATUS_BAR_TOP_MARGIN: f32 = 16.0;
const PLAYER_STATUS_BAR_LEFT_MARGIN: f32 = 16.0;

const HEALTH_BAR_WIDTH: f32 = 256.0;
const HEALTH_BAR_HEIGHT: f32 = 64.0;

const HEALTH_INDICATOR_WIDTH: f32 = 64.0;
const HEALTH_INDICATOR_HEIGHT: f32 = 64.0;

struct Scoreboard;

struct GameStateLabel;

struct HealthIndicator {
    pub health: u8,
}

struct ActiveEffectsBar;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage_after("startup", STARTUP_STAGE)
            .add_startup_system_to_stage(STARTUP_STAGE, setup_scoreboard.system())
            .add_startup_system_to_stage(STARTUP_STAGE, setup_health_bar.system())
            .add_startup_system_to_stage(STARTUP_STAGE, setup_game_status.system())
            .add_system(update_scoreboard.system())
            .add_system(update_health_bar.system())
            .add_system(update_active_effects.system())
            .add_system(update_game_state_screen.system());
    }
}

fn setup_scoreboard(commands: &mut Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn((Scoreboard,)).with_bundle(TextBundle {
        text: Text {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            style: TextStyle {
                color: Color::rgb(0.5, 0.5, 0.5),
                font_size: 40.0,
                ..Default::default()
            },
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn update_scoreboard(game: Res<Game>, mut query: Query<&mut Text, With<Scoreboard>>) {
    for mut text in query.iter_mut() {
        (*text).value = format!("Score: {}. Best score: {}", game.score, game.best_score);
    }
}

fn setup_health_bar(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Player>,
) {
    let health_handle = asset_server.load("sprites/health.png");
    let material_handle = materials.add(health_handle.into());

    commands
        .spawn(NodeBundle {
            material: materials.add(Color::NONE.into()),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(16.0),
                    top: Val::Px(16.0),
                    ..Default::default()
                },
                size: Size {
                    height: Val::Px(HEALTH_BAR_HEIGHT),
                    ..Default::default()
                },
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(move |parent| {
            for player in player_query.iter() {
                for health in 1..=player.max_health {
                    parent
                        .spawn(ImageBundle {
                            style: Style {
                                max_size: Size::new(
                                    Val::Px(HEALTH_INDICATOR_WIDTH),
                                    Val::Px(HEALTH_INDICATOR_HEIGHT),
                                ),
                                margin: Rect {
                                    right: Val::Px(16.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            material: material_handle.clone(),
                            draw: Draw {
                                is_transparent: true,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with(HealthIndicator { health });
                }
            }
        })
        .spawn(TextBundle {
            text: Text {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    font_size: 32.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(PLAYER_STATUS_BAR_LEFT_MARGIN),
                    top: Val::Px(
                        PLAYER_STATUS_BAR_TOP_MARGIN
                            + HEALTH_INDICATOR_HEIGHT
                            + PLAYER_STATUS_BAR_TOP_MARGIN,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with(ActiveEffectsBar);
}

fn update_health_bar(
    player_query: Query<&Player>,
    mut health_bar_query: Query<(&HealthIndicator, &mut Draw, &mut Transform)>,
) {
    for player in player_query.iter() {
        for (health_indicator, mut draw, mut transform) in health_bar_query.iter_mut() {
            let dx;
            let dy;
            if health_indicator.health > player.health {
                dx = -0.05;
                dy = -0.05;
            } else {
                dx = 0.05;
                dy = 0.05;
            }

            transform.scale.x = (transform.scale.x + dx).min(1.0).max(0.0);
            transform.scale.y = (transform.scale.y + dy).min(1.0).max(0.0);

            if transform.scale.x == 0.0 && transform.scale.y == 0.0 {
                if draw.is_visible {
                    draw.is_visible = false;
                }
            } else if !draw.is_visible {
                draw.is_visible = true;
            }
        }
    }
}

fn update_active_effects(
    active_effects: Query<&ActiveEffects, With<Player>>,
    mut active_effects_bar: Query<&mut Text, With<ActiveEffectsBar>>,
) {
    for mut text in active_effects_bar.iter_mut() {
        let text = &mut *text;
        text.value.clear();

        let mut effects = String::new();
        for active_effects in active_effects.iter() {
            for effect in &active_effects.effects {
                if !effect.is_active() {
                    continue;
                }
                let effect_text = match effect.length {
                    EffectLength::Permanent => effect.name.clone(),
                    EffectLength::Temporary(time_left) => {
                        format!("{}: {:.2} ms left", effect.name, time_left)
                    }
                    EffectLength::Countable(count_left) => {
                        format!("{}: {} left", effect.name, count_left)
                    }
                };
                effects.push_str(&format!("{}\n", effect_text));
            }
        }

        text.value = effects;
    }
}

fn update_game_state_screen(
    game: Res<Game>,
    mut query: Query<(&mut Text, &mut Draw), With<GameStateLabel>>,
) {
    for (mut text, mut draw) in query.iter_mut() {
        match game.state {
            GameState::WaitingForStart => {
                draw.is_visible = true;
                (*text).value = "Press Space to start".to_string();
            }
            GameState::Running => {
                draw.is_visible = false;
                (*text).value = "".to_string();
            }
            GameState::Paused => {
                draw.is_visible = true;
                (*text).value = "Paused".to_string();
            }
            GameState::GameOver => {
                draw.is_visible = true;
                (*text).value =
                    format!("Game over!\nYour score: {}\nPress R to restart", game.score);
            }
        }
    }
}

fn setup_game_status(commands: &mut Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn((GameStateLabel,)).with_bundle(TextBundle {
        text: Text {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            style: TextStyle {
                color: Color::rgb(0.5, 0.5, 0.5),
                font_size: 120.0,
                ..Default::default()
            },
            ..Default::default()
        },
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}
