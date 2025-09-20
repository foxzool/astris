use crate::{PlayerAction, PlayerControlled};
use astris_actor::Actor;
use astris_camera::FollowTarget;
use astris_core::components::{Faction, FactionKind};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use leafwing_input_manager::prelude::*;

pub(crate) fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let scene: Handle<Scene> = asset_server.load("models/base_character/base.glb#Scene0");
    // let idle_anim = asset_server.load("models/base_character/base.glb#Animation0");

    let _player = commands
        .spawn((
            Mesh3d(meshes.add(Capsule3d {
                radius: 0.5,
                half_length: 0.5,
            })),
            MeshMaterial3d(materials.add(Color::from(bevy::color::palettes::css::DARK_CYAN))),
            Transform::from_xyz(0.0, 2.0, 0.0),
            // The player character needs to be configured as a dynamic rigid body of the physics
            // engine.
            RigidBody::Dynamic,
            Collider::capsule(0.5, 1.0),
            // This is Tnua's interface component.
            TnuaController::default(),
            // A sensor shape is not strictly necessary, but without it we'll get weird results.
            TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
            // Tnua can fix the rotation, but the character will still get rotated before it can do so.
            // By locking the rotation we can prevent this.
            LockedAxes::ROTATION_LOCKED,
            PlayerControlled,
            FollowTarget,
            Actor,
            SceneRoot(scene),
            Faction(FactionKind::Player),
            PlayerControlled::default_input_map(),
        ))
        .id();
}

pub(crate) fn cast_fireball(
    action_state: Single<&ActionState<PlayerAction>, With<PlayerControlled>>,
) {
    if action_state.just_pressed(&PlayerAction::Ability1) {
        println!("Fwoosh!");
    }
}

pub(crate) fn apply_controls(
    action_state: Single<(&ActionState<PlayerAction>, &mut TnuaController), With<PlayerControlled>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    let mut direction_vector = Vec2::ZERO;

    let (action_state, mut controller) = action_state.into_inner();

    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.pressed(&input_direction)
            && let Some(direction) = input_direction.direction()
        {
            // Sum the directions as 2D vectors
            direction_vector += *direction;
        }
    }

    // 将输入归一化，避免对角线比直线移动更快。
    let input_dir = direction_vector.normalize_or_zero();

    // 默认使用世界坐标系；如果有摄像机，则按屏幕方向（摄像机朝向）投影到地面。
    let mut desired_velocity = Vec3::new(input_dir.x, 0.0, input_dir.y);
    if let Some(camera_tf) = camera_query.iter().next() {
        // 摄像机前方/右方投影到 XZ 平面，和输入组合得到屏幕上下左右。
        let forward = camera_tf.forward();
        let right = camera_tf.right();
        let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
        let right_flat = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

        desired_velocity =
            (forward_flat * input_dir.y + right_flat * input_dir.x).normalize_or_zero();
    }
    desired_velocity *= 10.0;

    // 每帧都要喂入基础运动信息；若没有输入则提供零速度，防止控制器失效后角色下落。
    controller.basis(TnuaBuiltinWalk {
        // desired_velocity 决定角色移动方向与速度。
        desired_velocity,
        // float_height 需略大于角色中心到底部碰撞体的距离，否则无法稳定悬浮。
        float_height: 1.5,
        // 其余参数保持默认值，可按需在文档中查阅说明进行调整。
        ..Default::default()
    });
}
