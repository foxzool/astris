use bevy::prelude::*;

pub(crate) fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 怪物：红色立方体，随机摆几个
    let monster_positions = [
        Vec3::new(5.0, 0.5, 5.0),
        Vec3::new(-5.0, 0.5, -3.0),
        Vec3::new(8.0, 0.5, -6.0),
    ];

    for pos in monster_positions {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.6, 0.2, 0.2))),
            Transform::from_translation(pos),
        ));
    }

    // 地板：灰色平面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(60.0, 60.0))),
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
