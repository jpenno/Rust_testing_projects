use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::SimulationState;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((bullet_move, remove_bullet).in_set(OnUpdate(SimulationState::Running)));
    }
}
