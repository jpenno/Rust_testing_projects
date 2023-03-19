use bevy::prelude::*;

pub const PLAYER_SHOOT_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct PlayerShootTimer {
    pub timer: Timer,
}

impl Default for PlayerShootTimer {
    fn default() -> Self {
        PlayerShootTimer {
            timer: Timer::from_seconds(PLAYER_SHOOT_TIME, TimerMode::Once),
        }
    }
}
