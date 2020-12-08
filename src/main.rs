mod awards;
mod effects;
mod enemies;
mod game;
mod player;
mod systems;
mod world;

use crate::awards::AwardTimer;
use crate::effects::{ActiveEffects, VisualEffects};
use crate::enemies::SpawnTimer;
use crate::game::{Game, GameStateEvent};
use crate::player::{Player, PlayerEvent};
use crate::systems::debug::DebugPlugin;
use crate::systems::hud::HudPlugin;
use crate::world::{AffectedByGravity, Gravity, Velocity};
use bevy::prelude::*;

fn main() {
    let mut app = App::build();

    app.add_resource(WindowDescriptor {
        width: world::SCREEN_WIDTH,
        height: world::SCREEN_HEIGHT,
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);

    app.add_event::<GameStateEvent>()
        .add_event::<PlayerEvent>()
        .add_startup_system(setup.system())
        .add_plugin(HudPlugin)
        .add_system(systems::input::input.system())
        .add_system(systems::spawning::drop_enemies.system())
        .add_system(systems::spawning::spawn_health.system())
        .add_system(systems::gameplay::cleanup_effects.system())
        .add_system(systems::visual_effects::run_visual_effects.system())
        .add_system(systems::spawning::spawn_new_enemy.system())
        .add_system(systems::physics::movement.system())
        .add_system(systems::physics::gravity.system())
        .add_system(systems::physics::collisions.system())
        .add_system(systems::events::player_events.system())
        .add_system(systems::events::game_state_events.system());

    #[cfg(feature = "debug_panel")]
    app.add_plugin(DebugPlugin);

    app.run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(Game::new())
        .insert_resource(Gravity::default())
        .insert_resource(SpawnTimer {
            timer: Timer::from_seconds(3.0, true),
        })
        .insert_resource(AwardTimer::new(5.0, 15.0))
        .spawn((Player::new(),))
        .with(ActiveEffects::new())
        .with(VisualEffects::new())
        .with(Velocity(Vec2::new(0.0, 0.0)))
        .with(AffectedByGravity)
        .with_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(player::WIDTH, player::HEIGHT)),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_translation(Vec3::new(
                player::INITIAL_POSITION_X,
                player::INITIAL_POSITION_Y,
                0.0,
            )),
            ..Default::default()
        })
        .spawn(SpriteBundle {
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
