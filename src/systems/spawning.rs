use bevy::prelude::*;
use crate::enemies::{SpawnTimer, Enemy, Award, GivesAward};
use crate::game::{Game};
use crate::world::{Velocity, Collidable};
use crate::enemies;
use rand::{thread_rng, Rng};

pub fn spawn_new_enemy(
    commands: &mut Commands,
    time: Res<Time>,
    game: Res<Game>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    if !game.is_running() {
        return;
    }

    spawn_timer.timer.tick(time.delta_seconds);

    if !spawn_timer.timer.finished {
        return;
    }

    spawn_timer.timer.duration = thread_rng().gen_range(2.0, 3.0);
    let mut rng = thread_rng();

    commands
        .spawn((Enemy, ))
        .with(GivesAward::new(Award::Score(enemies::SCORE)))
        .with(Velocity(Vec2::new(-enemies::VELOCITY_X, enemies::VELOCITY_Y)))
        .with(Collidable)
        .with_bundle(SpriteComponents {
            sprite: Sprite::new(Vec2::new(enemies::WIDTH, enemies::HEIGHT)),
            material: materials.add(Color::rgb(
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0),
            ).into()),
            transform: Transform::from_translation(Vec3::new(enemies::INITIAL_POSITION_X, enemies::INITIAL_POSITION_Y, 0.0)),
            ..Default::default()
        });
}

pub fn drop_enemies(
    commands: &mut Commands,
    game_window: Res<WindowDescriptor>,
    game: Res<Game>,
    query: Query<(Entity, &Sprite, &Transform), With<Enemy>>
) {
    if !game.is_running() {
        return;
    }

    for (enemy_entity, sprite, transform) in query.iter() {
        if transform.translation.x() + sprite.size.x() < -(game_window.width as f32) / 2.0 {
            commands.despawn(enemy_entity);
        }
    }
}
