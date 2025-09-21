use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

mod components;
mod input;
mod systems;

pub use components::*;
pub use input::*;
pub(crate) use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<InputManagerPlugin<PlayerAction>>() {
            app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
        }

        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (apply_controls, cast_fireball, draw_player_gizmos));
    }
}
