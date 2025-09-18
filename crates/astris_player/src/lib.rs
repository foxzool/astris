use bevy::prelude::*;

mod components;
mod systems;

pub use components::*;
pub(crate) use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}
