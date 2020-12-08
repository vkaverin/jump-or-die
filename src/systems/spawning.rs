use crate::awards::{Award, AwardTimer};
use crate::enemies;
use crate::enemies::{Enemy, SpawnTimer};
use crate::game::Game;
use crate::player::Player;
use crate::world::{Collider, Velocity};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub fn spawn_new_enemy(
    commands: &mut Commands,
    window: Res<WindowDescriptor>,
    time: Res<Time>,
    game: Res<Game>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Sprite, With<Player>>,
) {
    if !game.is_running() {
        return;
    }

    spawn_timer.timer.tick(time.delta_seconds());

    if !spawn_timer.timer.finished() {
        return;
    }

    spawn_timer
        .timer
        .set_duration(thread_rng().gen_range(2.0, 3.0));
    let mut rng = thread_rng();

    commands
        .spawn((Enemy,))
        .with(Velocity(Vec2::new(
            -enemies::VELOCITY_X,
            enemies::VELOCITY_Y,
        )))
        .with(Collider::Solid)
        .with_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(enemies::WIDTH, enemies::HEIGHT)),
            material: materials.add(
                Color::rgb(
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                )
                .into(),
            ),
            transform: Transform::from_translation(Vec3::new(
                enemies::INITIAL_POSITION_X,
                enemies::INITIAL_POSITION_Y,
                0.0,
            )),
            ..Default::default()
        });

    for sprite in player_query.iter() {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite::new(Vec2::new(enemies::WIDTH, window.height as f32)),
                material: materials.add(Color::NONE.into()),
                transform: Transform::from_translation(Vec3::new(
                    enemies::INITIAL_POSITION_X + sprite.size.x + 1.0,
                    enemies::INITIAL_POSITION_Y,
                    0.0,
                )),
                draw: Draw {
                    is_visible: false,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Velocity(Vec2::new(
                -enemies::VELOCITY_X,
                enemies::VELOCITY_Y,
            )))
            .with(Collider::Award(Award::Score(enemies::SCORE)));
    }
}

pub fn drop_enemies(
    commands: &mut Commands,
    game_window: Res<WindowDescriptor>,
    game: Res<Game>,
    query: Query<(Entity, &Sprite, &Transform), With<Enemy>>,
) {
    if !game.is_running() {
        return;
    }

    for (enemy_entity, sprite, transform) in query.iter() {
        if transform.translation.x + sprite.size.x < -(game_window.width as f32) / 2.0 {
            commands.despawn(enemy_entity);
        }
    }
}

pub fn spawn_health(
    commands: &mut Commands,
    window: Res<WindowDescriptor>,
    time: Res<Time>,
    game: Res<Game>,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<AwardTimer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    players: Query<&Player>,
) {
    if !game.is_running() {
        return;
    }

    for player in players.iter() {
        if player.health == player.max_health {
            return;
        }
    }

    timer.timer.tick(time.delta_seconds());
    if !timer.timer.finished() {
        return;
    }

    timer.refill();

    let mut rng = rand::thread_rng();
    let mut health: u8 = 1;
    if rng.gen_bool(0.25) {
        health = 2;
    }

    let texture_handle = asset_server.load("sprites/health.png");
    let width = 48.0 + (health - 1) as f32 * 16.0;
    let height = 48.0 + (health - 1) as f32 * 16.0;

    let initial_x = (window.width as f32 + width) / 2.0;
    let initial_y = height / 2.0;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite::new(Vec2::new(width, height)),
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(initial_x, initial_y, 0.0)),
            ..Default::default()
        })
        .with(Velocity(Vec2::new(-300.0, 0.0)))
        .with(Collider::Award(Award::Health(health)));
}
