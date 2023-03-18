use bevy::{prelude::*, window::PrimaryWindow};

mod player;

use crate::player::*;

fn main() {
    App::new()
        // add Plugins
        // .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Shooter Game".into(),
                resolution: (700., 900.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(PlayerPlugin)
        // add start up systems
        .add_startup_system(spawn_camera)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
