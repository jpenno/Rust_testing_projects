use bevy::prelude::*;
use rand::prelude::*;

use crate::bullet::components::*;
use crate::enemy::ENEMY_SIZE;

pub const BULLET_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub size: f32,
    pub direction: Vec3,
    pub shoot_timer: Timer,
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
            shoot_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

impl Enemy {
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, window: &Window) {
        let max_x = window.width() - ENEMY_SIZE;
        let spawn_x = rand::thread_rng().gen_range(ENEMY_SIZE..max_x);
        let spawn_y = window.height() + ENEMY_SIZE + 50.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(spawn_x, spawn_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy { ..default() },
        ));
    }

    pub fn upate_timers(&mut self, time: &Res<Time>) {
        self.shoot_timer.tick(time.delta());
    }

    pub fn shoot(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        enemy_transform: &Transform,
    ) {
        if self.shoot_timer.finished() {
            println!("Enemy Shoot");
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        enemy_transform.translation.x,
                        enemy_transform.translation.y + 64.0 / 2.0,
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
                Bullet {
                    size: BULLET_SIZE,
                    ..default()
                },
            ));
        }
    }
}
