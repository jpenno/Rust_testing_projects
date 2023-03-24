use bevy::prelude::*;
use rand::prelude::*;

use crate::enemy::ENEMY_SIZE;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub size: f32,
    pub direction: Vec3,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            speed: 100.0,
            size: 64.0,
            direction: Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
        }
    }
}

impl Enemy {
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, window: &Window) {
        let max_x = window.width() - ENEMY_SIZE;
        let random_x = rand::thread_rng().gen_range(64.0..max_x);
        let random_y = window.height() + ENEMY_SIZE + 50.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy { ..default() },
        ));
    }
}
