use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::SimulationState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerShootTimer>()
            .add_startup_system(spawn_player)
            .add_systems(
                (
                    player_movement,
                    confine_player_movement,
                    player_shoot,
                    tick_player_shoot_timer,
                    player_hit_enemy,
                )
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}
