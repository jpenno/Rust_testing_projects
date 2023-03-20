use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemys)
            .add_system(move_enemys);
    }
}
