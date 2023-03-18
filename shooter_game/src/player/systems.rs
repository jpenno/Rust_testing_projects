use bevy::{prelude::*, window::PrimaryWindow};

use crate::bullet::components::*;
use crate::player::components::*;

pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SLOW_SPEED: f32 = 200.0;
pub const BULLET_SIZE: f32 = 32.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    println!("test");

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, PLAYER_SIZE, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            // code to change the size of a sprite
            // sprite: Sprite {
            //     custom_size: Some(Vec2::new(500.0, 500.0)),
            //     ..default()
            // },
            ..default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = player_query.get_single_mut() else { return };

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    let player_speed = if keyboard_input.pressed(KeyCode::LShift) {
        PLAYER_SLOW_SPEED
    } else {
        PLAYER_SPEED
    };

    transform.translation += direction * player_speed * time.delta_seconds();
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else { return };
    let window = window_query.get_single().unwrap();

    let half_player_size = PLAYER_SIZE / 2.0;
    let x_min = 0.0 + half_player_size;
    let x_max = window.width() - half_player_size;
    let y_min = 0.0 + half_player_size;
    let y_max = window.height() - half_player_size;

    let mut translation = player_transform.translation;

    if translation.x < x_min {
        translation.x = x_min
    } else if translation.x > x_max {
        translation.x = x_max;
    }
    if translation.y < y_min {
        translation.y = y_min
    } else if translation.y > y_max {
        translation.y = y_max;
    }

    player_transform.translation = translation;
}

pub fn player_shoot(
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let Ok(player_transform) = player_query.get_single() else {return};
    let player_translation = player_transform.translation;

    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    player_translation.x,
                    player_translation.y + PLAYER_SIZE / 2.0,
                    0.0,
                ),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                // code to change the size of a sprite
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                    ..default()
                },
                ..default()
            },
            Bullet { ..default() },
        ));
    }
}
