use bevy::prelude::*;

mod bullet;
mod enemy;
mod player;
mod systems;

use crate::systems::*;
use crate::bullet::*;
use crate::enemy::*;
use crate::player::*;

fn main() {
    App::new()
        // add states
        .add_state::<SimulationState>()
        // add Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (700.0, 900.0).into(),
                title: "Shooter Game".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BulletPlugin)
        // add start up systems
        .add_startup_system(spawn_camera)
        // add systems
        .add_system(toggle_simulation)
        .add_system(exit_game)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
