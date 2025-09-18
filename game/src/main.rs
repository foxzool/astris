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
        .run();
}
