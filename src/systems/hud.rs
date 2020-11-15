use crate::game::{Game, GameState};
use bevy::prelude::*;
use crate::player::Player;

const STARTUP_STAGE: &str = "hud_startup";

struct Scoreboard;

struct GameStateLabel;

struct HealthBar {
    pub heath: u8,
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage_after("startup", STARTUP_STAGE)
            .add_startup_system_to_stage(STARTUP_STAGE, setup_scoreboard.system())
            .add_startup_system_to_stage(STARTUP_STAGE, setup_health_bar.system())
            .add_startup_system_to_stage(STARTUP_STAGE, setup_game_status.system())
            .add_system(update_scoreboard.system())
            .add_system(update_health_bar.system())
            .add_system(update_game_state_screen.system());
    }
}

fn setup_scoreboard(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
) {
    commands.spawn((Scoreboard,))
        .with_bundle(TextComponents {
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
    game_window: Res<WindowDescriptor>,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Player>
) {
    let health_handle = asset_server.load("sprites/health.png");
    let material_handle = materials.add(health_handle.into());
    let screen_padding = 16.0;

    for player in player_query.iter() {
        let health_sprite_width = 64.0;
        for health in 1..=player.max_health {
            let sprite_padding = screen_padding
                + health_sprite_width / 2.0
                + (health - 1) as f32 * 10.0;
            let sprites_previous_width = (health - 1) as f32 * health_sprite_width;
            let x = game_window.width as f32 / 2.0
                - sprite_padding
                - sprites_previous_width;
            commands.spawn((HealthBar {
                heath: health
            }, ))
                .with_bundle(SpriteComponents {
                    sprite: Sprite::new(Vec2::splat(64.0)),
                    material: material_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(x, game_window.height as f32 / 2.0 - 100.0, 0.0)),
                    ..Default::default()
                });
        }
    }
}

fn update_health_bar(
    player_query: Query<&Player>,
    mut health_bar_query: Query<(&HealthBar, &mut Draw)>
) {
    for player in player_query.iter() {
        for (health_bar, mut draw) in health_bar_query.iter_mut() {
            draw.is_visible = health_bar.heath <= player.health;
        }
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

fn setup_game_status(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
) {
    commands.spawn((GameStateLabel,))
        .with_bundle(TextComponents {
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
