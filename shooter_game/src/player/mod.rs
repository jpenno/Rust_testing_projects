use bevy::prelude::*;

pub mod components;
mod systems;
pub mod resources;

use systems::*;
use resources::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerShootTimer>()
            .add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(player_shoot)
            .add_system(tick_player_shoot_timer);
    }
}

// componets
