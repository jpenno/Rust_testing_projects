use bevy::{prelude::*, window::PrimaryWindow};

mod bullet;
mod enemy;
mod player;

use crate::bullet::*;
use crate::enemy::*;
use crate::player::*;

fn main() {
    App::new()
        // add states
        .add_state::<SimulationState>()
        // add Plugins
        // .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Shooter Game".into(),
                resolution: (700.0, 900.).into(),
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
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Escape) {
        return;
    }

    match simulation_state.0 {
        SimulationState::Running => {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Simulation Paused");
        }
        SimulationState::Paused => {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Simulation Running");
        }
    }
}
