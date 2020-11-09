mod game;
mod player;
mod systems;
mod world;

use crate::game::Game;
use crate::player::Player;
use crate::systems::debug::DebugPlugin;
use crate::world::{AffectedByGravity, Collidable, Gravity, Velocity};
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

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
        .add_startup_system(setup.system())
        .add_system(systems::input::input.system())
        .add_system(systems::physics::movement.system())
        .add_system(systems::physics::gravity.system())
        // .add_system(collisions.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(Game::new())
        .insert_resource(Gravity::default())
        .spawn((Player::new(), ))
            .with(Velocity(Vec2::new(0.0, 0.0)))
            .with(AffectedByGravity)
            .with_bundle(SpriteComponents {
                sprite: Sprite::new(Vec2::new(player::WIDTH, player::HEIGHT)),
                material: materials.add(Color::BLACK.into()),
                transform: Transform::from_translation(Vec3::new(player::INITIAL_POSITION_X, player::INITIAL_POSITION_Y, 0.0)),
                ..Default::default()
            })
        .spawn((Collidable, ))
            .with_bundle(SpriteComponents {
                sprite: Sprite::new(Vec2::new(world::SCREEN_WIDTH as f32, world::SCREEN_HEIGHT as f32)),
                material: materials.add(Color::BLACK.into()),
                transform: Transform::from_translation(Vec3::new(0.0, -((world::SCREEN_HEIGHT / 2) as f32), 0.0)),
                ..Default::default()
            })
        // .spawn((Collidable, ))
        //     .with(HasFriction)
        //     .with_bundle(SpriteComponents {
        //     sprite: Sprite::new(Vec2::new(SQUARE_SIZE * 5.0, SQUARE_SIZE / 2.0)),
        //     material: materials.add(Color::BLACK.into()),
        //     transform: Transform::from_translation(Vec3::new(SQUARE_SIZE * 5.0, SQUARE_SIZE, 0.0)),
        //     ..Default::default()
        // })
    ;
}
