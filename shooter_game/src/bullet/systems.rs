use bevy::prelude::*;

use crate::bullet::components::*;


pub fn bullet_move(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut bullet_transform, bullet) in bullet_query.iter_mut() {
        bullet_transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
    }
}
