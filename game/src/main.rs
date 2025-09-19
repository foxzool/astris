use astris_player::PlayerAction;
use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use leafwing_input_manager::prelude::InputManagerPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Astris Game".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: "../assets".to_string(),
                    ..Default::default()
                }),
        )
        .add_plugins((
            InputManagerPlugin::<PlayerAction>::default(),
            PhysicsPlugins::default(),
            // We need both Tnua's main controller plugin, and the plugin to connect to the physics
            // backend (in this case Avian 3D)
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
        ))
        .add_plugins(astris_actor::ActorPlugin)
        .add_plugins(astris_camera::CameraPlugin)
        .add_plugins(astris_player::PlayerPlugin)
        .add_plugins(astris_map::MapPlugin)
        .run();
}
