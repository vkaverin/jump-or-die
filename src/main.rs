mod enemies;
mod game;
mod player;
mod systems;
mod world;
mod effects;

use crate::enemies::SpawnTimer;
use crate::game::{Game, GameStateEvent};
use crate::player::{Player, PlayerEvent};
use crate::systems::debug::DebugPlugin;
use crate::world::{AffectedByGravity, Gravity, Velocity};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use crate::systems::hud::HudPlugin;
use crate::effects::Effects;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            width: world::SCREEN_WIDTH,
            height: world::SCREEN_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_event::<GameStateEvent>()
        .add_event::<PlayerEvent>()
        .add_startup_system(setup.system())
        .add_plugin(HudPlugin)
        .add_system(systems::input::input.system())
        .add_system(systems::spawning::drop_enemies.system())
        .add_system(systems::gameplay::cleanup_effects.system())
        .add_system(systems::spawning::spawn_new_enemy.system())
        .add_system(systems::physics::movement.system())
        .add_system(systems::physics::gravity.system())
        .add_system(systems::physics::collisions.system())
        .add_system(systems::awards::collect_enemy_awards.system())
        .add_system(systems::events::player_events.system())
        .add_system(systems::events::game_state_events.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(Game::new())
        .insert_resource(Gravity::default())
        .insert_resource(SpawnTimer {
            timer: Timer::from_seconds(3.0, true),
        })
        .spawn((Player::new(),))
        .with(Effects::new())
        .with(Velocity(Vec2::new(0.0, 0.0)))
        .with(AffectedByGravity)
        .with_bundle(SpriteComponents {
            sprite: Sprite::new(Vec2::new(player::WIDTH, player::HEIGHT)),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_translation(Vec3::new(
                player::INITIAL_POSITION_X,
                player::INITIAL_POSITION_Y,
                0.0,
            )),
            ..Default::default()
        })
        .spawn(SpriteComponents {
            sprite: Sprite::new(Vec2::new(
                world::SCREEN_WIDTH as f32,
                world::SCREEN_HEIGHT as f32,
            )),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                -((world::SCREEN_HEIGHT / 2) as f32),
                0.0,
            )),
            ..Default::default()
        });
}
