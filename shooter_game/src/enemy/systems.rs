use bevy::{prelude::*, window::PrimaryWindow};

use crate::{bullet::components::Bullet, enemy::components::*};

pub fn spawn_enemys(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() - 100.0, 0.0),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..default()
        },
        Enemy { ..default() },
    ));
}

pub fn move_enemys(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation += enemy.direction * enemy.speed * time.delta_seconds();
    }
}

pub fn enemy_hit_bullet(
    mut commands: Commands,
    bullet_query: Query<(&Transform, Entity, &Bullet)>,
    enemy_query: Query<(&Transform, Entity, &Enemy)>,
) {
    for (enemy_transform, enemy_entity, enemy) in enemy_query.iter() {
        for (bullet_transform, bullet_entity, bullet) in bullet_query.iter() {
            let distance = enemy_transform
                .translation
                .distance(bullet_transform.translation);

            let bullet_radius = bullet.size / 2.0;
            let enemy_radius = enemy.size / 2.0;
            if distance < enemy_radius + bullet_radius {
                println!("Bullet hit enemy");
                commands.entity(enemy_entity).despawn();
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}
