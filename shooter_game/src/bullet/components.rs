use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
}

impl Default for Bullet {
    fn default() -> Self {
        Bullet {
            direction: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
            speed: 700.0,
        }
    }
}
