use bevy::prelude::*;

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
        .add_plugins(astris_actor::ActorPlugin)
        .add_plugins(astris_camera::CameraPlugin)
        .add_plugins(astris_player::PlayerPlugin)
        .add_plugins(astris_map::MapPlugin)
        .run();
}
