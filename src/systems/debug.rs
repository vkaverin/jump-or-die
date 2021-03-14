use crate::world::Velocity;

use crate::game::Game;
use crate::player::Player;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(debug_setup.system())
            .add_system(update_debug_info_panel.system());
    }
}

pub struct DebugBlock;
struct DebugText;

fn debug_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(256.0), Val::Percent(100.0)),
                border: Rect::all(Val::Px(2.0)),
                ..Default::default()
            },
            material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.9).into()),
            visible: Visible {
                is_transparent: true,
                is_visible: false,
            },
            ..Default::default()
        })
        .with(DebugBlock)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::with_section(
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 15.0,
                        color: Color::WHITE,
                    },
                    Default::default()
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(5.0),
                        left: Val::Px(5.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                visible: Visible {
                    is_visible: false,
                    is_transparent: true,
                },
                ..Default::default()
            }).with(DebugText);
        });
}

fn update_debug_info_panel(
    diagnostics: Res<Diagnostics>,
    game: Res<Game>,
    player_query: Query<(&Player, &Velocity, &Transform)>,
    mut text_query: Query<&mut Text, With<DebugText>>,
) {
    for mut text in text_query.iter_mut() {
        for (player, velocity, transform) in player_query.iter() {
            text.sections[0].value = format!(
                "Game: {:#?}\nPlayer: {:#?}\nVelocity: {:#?}\n Translation: {:#?}\n",
                game, player, velocity, transform.translation.truncate()
            );

            if let Some(measurement) =
            diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FPS)
            {
                text.sections[0].value
                    .push_str(&format!("\nFPS: {:.2}", measurement.value));
            }
            if let Some(measurement) =
            diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FRAME_TIME)
            {
                text.sections[0].value
                    .push_str(&format!("\nframe time: {:.3}", measurement.value));
            }
            if let Some(measurement) =
            diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
            {
                text.sections[0].value
                    .push_str(&format!("\nframes count: {}", measurement.value));
            }
        }
    }
}
