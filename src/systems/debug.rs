use crate::world::{Gravity, Velocity};

use crate::player::Player;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use crate::game::Game;

pub struct DebugUiPanel;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(debug_setup.system())
            .add_system(print_debug_data.system());
    }
}

fn debug_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((DebugUiPanel,)).with_bundle(TextComponents {
        text: Text {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            style: TextStyle {
                color: Color::rgb(0.5, 0.5, 0.5),
                font_size: 20.0,
            },
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn print_debug_data(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    game: Res<Game>,
    gravity: Res<Gravity>,
    info_query: Query<(&Player, &Velocity, &Transform)>,
    mut ui_query: Query<(&DebugUiPanel, &mut Text)>,
) {
    for (_panel, mut text) in ui_query.iter_mut() {
        for (player, velocity, transform) in info_query.iter() {
            text.value.truncate(0);
            text.value.push_str(&format!("{:?}", player));
            text.value.push_str(&format!("\n{:?}", game));
            text.value.push_str(&format!(
                "\nvelocity: (x = {},  y = {})",
                velocity.0.x(),
                velocity.0.y()
            ));
            text.value.push_str(&format!(
                "\nposition: (x = {},  y = {})",
                transform.translation.x(),
                transform.translation.y()
            ));
            text.value
                .push_str(&format!("\ngravity: {}", time.delta_seconds * gravity.0));
            if let Some(measurement) = diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FPS)
            {
                text.value
                    .push_str(&format!("\nFPS: {}", measurement.value));
            }
            if let Some(measurement) =
                diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FRAME_TIME)
            {
                text.value
                    .push_str(&format!("\nframe time: {}", measurement.value));
            }
            if let Some(measurement) =
                diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
            {
                text.value
                    .push_str(&format!("\nframes count: {}", measurement.value));
            }
        }
    }
}
