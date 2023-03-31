use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    bullet::components::{Bullet, BulletType},
    enemy::components::*,
};

use super::resources::EnemySpawnTimer;

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn tick_enemy_timers(mut enemy_query: Query<&mut Enemy>, time: Res<Time>) {
    for mut enemy in enemy_query.iter_mut() {
        enemy.upate_timers(&time);
    }
}

pub fn enemy_shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemy_query: Query<(&Enemy, &Transform)>,
) {
    for (enemy, enemy_transform) in enemy_query.iter() {
        enemy.shoot(&mut commands, &asset_server, enemy_transform);
    }
}

pub fn spawn_enemys_over_time(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if !enemy_spawn_timer.timer.finished() {
        return;
    }

    let window = window_query.get_single().unwrap();
    Enemy::spawn(commands, asset_server, window);
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
            if bullet.bullet_type != BulletType::Enemy {
                let distance = enemy_transform
                    .translation
                    .distance(bullet_transform.translation);

                let bullet_radius = bullet.size / 2.0;
                let enemy_radius = enemy.size / 2.0;
                if distance < enemy_radius + bullet_radius {
                    commands.entity(enemy_entity).despawn();
                    commands.entity(bullet_entity).despawn();
                }
            }
        }
    }
}
