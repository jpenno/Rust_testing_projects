use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::SimulationState;

pub struct EnemyPlugin;

pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<EnemySpawnTimer>()
            // Start up systems
            .add_startup_system(spawn_enemys)
            // Systems
            .add_systems(
                (
                    tick_enemy_spawn_timer,
                    spawn_enemys_over_time,
                    move_enemys,
                    enemy_hit_bullet,
                )
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}
