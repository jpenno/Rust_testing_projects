use bevy::prelude::*;

#[derive(PartialEq)]
pub enum BulletType {
    Player,
    Enemy,
    None,
}

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
    pub size: f32,
    pub bullet_type: BulletType,
}

impl Default for Bullet {
    fn default() -> Self {
        Bullet {
            direction: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            speed: 700.0,
            size: 32.0,
            bullet_type: BulletType::None,
        }
    }
}

impl Bullet {
    pub fn new(bullet_type: BulletType) -> Bullet {
        match bullet_type {
            BulletType::Player => Bullet {
                direction: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                speed: 700.0,
                size: 32.0,
                bullet_type,
            },
            BulletType::Enemy => Bullet {
                direction: Vec3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                speed: 200.0,
                size: 32.0,
                bullet_type,
            },
            BulletType::None => Bullet::default(),
        }
    }
}
