use crate::PlayerControlled;
use astris_actor::Actor;
use astris_core::components::{Faction, FactionKind};
use bevy::prelude::*;

pub(crate) fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene: Handle<Scene> = asset_server.load("models/base_character/base.glb#Scene0");
    // let idle_anim = asset_server.load("models/base_character/base.glb#Animation0");

    let _player = commands
        .spawn((
            PlayerControlled,
            Actor,
            SceneRoot(scene),
            Faction(FactionKind::Player),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();
}
