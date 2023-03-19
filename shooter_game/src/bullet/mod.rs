use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_move).add_system(remove_bullet);
    }
}
