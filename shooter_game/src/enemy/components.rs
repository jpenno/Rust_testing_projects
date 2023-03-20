use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub direction: Vec3,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            speed: 100.0,
            direction: Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
        }
    }
}
