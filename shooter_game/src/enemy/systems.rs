use bevy::{prelude::*, window::PrimaryWindow};

use crate::enemy::components::*;

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
