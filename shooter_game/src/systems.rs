use bevy::app::AppExit;

use crate::SimulationState;

use bevy::{prelude::*, window::PrimaryWindow};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if !keyboard_input.just_pressed(KeyCode::Escape) {
        return;
    }
    app_exit_event_writer.send(AppExit);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>
) {
    if !keyboard_input.just_pressed(KeyCode::Tab) {
        return;
    }

    match simulation_state.0 {
        SimulationState::Running => {
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation Paused");
        }
        SimulationState::Paused => {
            next_simulation_state.set(SimulationState::Running);
            println!("Simulation Running");
        }
    }
}
