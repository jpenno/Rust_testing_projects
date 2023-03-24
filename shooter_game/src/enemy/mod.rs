use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

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
            .add_system(tick_enemy_spawn_timer)
            .add_system(spawn_enemys_over_time)
            .add_system(move_enemys)
            .add_system(enemy_hit_bullet);
    }
}
