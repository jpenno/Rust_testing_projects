use bevy::{prelude::*, window::PrimaryWindow};

use crate::bullet::components::*;

pub fn bullet_move(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut bullet_transform, bullet) in bullet_query.iter_mut() {
        bullet_transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
    }
}

pub fn remove_bullet(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Bullet), With<Bullet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (enity, transform, bullet) in bullet_query.iter() {
        if transform.translation.y > window.height() + bullet.size {
            commands.entity(enity).despawn();
        }
    }
}
