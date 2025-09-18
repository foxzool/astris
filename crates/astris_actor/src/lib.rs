use bevy::prelude::*;

mod components;

pub use components::*;

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, _app: &mut App) {}
}
