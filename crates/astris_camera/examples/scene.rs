use astris_camera::{CameraPlugin, FollowTarget};
use bevy::input::ButtonInput;
use bevy::math::primitives::{Cuboid, Plane3d};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, follow_target_movement)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 玩家：绿色立方体
    commands.spawn((
        FollowTarget,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.8, 0.2))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // 怪物：红色立方体，随机摆几个
    let monster_positions = [
        Vec3::new(5.0, 0.5, 5.0),
        Vec3::new(-5.0, 0.5, -3.0),
        Vec3::new(8.0, 0.5, -6.0),
    ];

    for pos in monster_positions {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
            Transform::from_translation(pos),
        ));
    }

    // 地板：灰色平面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
    ));

    // 灯光
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
        Name::new("Main Directional Light"),
    ));
}

// 玩家 WASD 控制
fn follow_target_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<FollowTarget>>,
) {
    let Ok(mut follow_target_tf) = query.single_mut() else {
        return;
    };
    let mut dir = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        dir.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        dir.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    if dir != Vec3::ZERO {
        follow_target_tf.translation += dir.normalize() * 5.0 * time.delta_secs();
    }
}
