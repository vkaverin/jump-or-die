use crate::world::Velocity;

use crate::game::Game;
use crate::player::Player;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

const PANEL_WIDTH: f32 = 512.0;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(debug_setup.system())
            .add_system(update_debug_info_panel.system());
    }
}

pub struct DebugPanel;

fn debug_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(PANEL_WIDTH), Val::Percent(100.0)),
                ..Default::default()
            },
            material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.9).into()),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(DebugPanel)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::with_section(
                    "",
                    TextStyle {
                        color: Color::WHITE,
                        font_size: 20.0,
                        ..Default::default()
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
                    is_transparent: true,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn update_debug_info_panel(
    diagnostics: Res<Diagnostics>,
    game: Res<Game>,
    info_query: Query<(&Player, &Velocity, &Transform)>,
    debug_panels_query: Query<&Children, With<DebugPanel>>,
    mut text_query: Query<&mut Text>,
) {
    for children in debug_panels_query.iter() {
        if let Ok(mut text) = text_query.get_mut(children[0]) {
            for (player, velocity, transform) in info_query.iter() {
                let text = &mut *text;
                text.sections[0].value.clear();
                text.sections[0].value.push_str(&format!("{:#?}", player));
                text.sections[0].value.push_str(&format!("\n{:#?}", *game));
                text.sections[0].value
                    .push_str(&format!("\nvelocity: {:?})", velocity.0));
                text.sections[0].value.push_str(&format!(
                    "\nposition: {:?}",
                    transform.translation.truncate()
                ));

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
}
