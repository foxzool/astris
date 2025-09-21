use crate::{PlayerAction, PlayerControlled};
use astris_actor::Actor;
use astris_camera::FollowTarget;
use astris_core::components::{Faction, FactionKind};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::window::PrimaryWindow;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use leafwing_input_manager::prelude::*;

const PLAYER_COLLIDER_RADIUS: f32 = 0.5;
const PLAYER_COLLIDER_HALF_HEIGHT: f32 = 1.0;
/// 来自 `assets/models/base_character/base.ron` 的包围盒最小 Y 值，
/// 用于将模型脚底与物理碰撞体底部对齐。
const BASE_CHARACTER_FOOT_OFFSET: f32 = 0.000_461_441;

pub(crate) fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene: Handle<Scene> = asset_server.load("models/base_character/base.glb#Scene0");
    // let idle_anim = asset_server.load("models/base_character/base.glb#Animation0");

    let _player = commands
        .spawn((
            // Mesh3d(meshes.add(Capsule3d {
            //     radius: 0.5,
            //     half_length: 0.5,
            // })),
            // MeshMaterial3d(materials.add(Color::from(bevy::color::palettes::css::DARK_CYAN))),
            Transform::from_xyz(0.0, 2.0, 0.0),
            // The player character needs to be configured as a dynamic rigid body of the physics
            // engine.
            RigidBody::Dynamic,
            Collider::capsule(PLAYER_COLLIDER_RADIUS, PLAYER_COLLIDER_HALF_HEIGHT),
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
            Faction(FactionKind::Player),
            Visibility::default(),
            PlayerControlled::default_input_map(),
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneRoot(scene),
                Transform {
                    translation: Vec3::new(
                        0.0,
                        -(PLAYER_COLLIDER_HALF_HEIGHT
                            + PLAYER_COLLIDER_RADIUS
                            + BASE_CHARACTER_FOOT_OFFSET),
                        0.0,
                    ),
                    rotation: Quat::from_rotation_y(std::f32::consts::PI),
                    ..Default::default()
                },
            ));
        })
        .id();
}

pub(crate) fn cast_fireball(
    action_state: Single<&ActionState<PlayerAction>, With<PlayerControlled>>,
) {
    if action_state.just_pressed(&PlayerAction::Ability1) {
        println!("Fwoosh!");
    }
}

/// 将角色朝向设置为鼠标在地面落点的方向。
pub(crate) fn face_player_to_cursor(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut player_query: Query<(&mut Transform, &GlobalTransform), With<PlayerControlled>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let mut chosen_camera: Option<(&Camera, &GlobalTransform)> = None;
    for (camera, transform) in camera_query.iter() {
        if camera.is_active {
            chosen_camera = Some((camera, transform));
            break;
        } else if chosen_camera.is_none() {
            chosen_camera = Some((camera, transform));
        }
    }

    let Some((camera, camera_transform)) = chosen_camera else {
        return;
    };

    let ray = match camera.viewport_to_world(camera_transform, cursor_position) {
        Ok(ray) => ray,
        Err(_) => return,
    };

    let Ok((mut player_transform, player_global)) = player_query.single_mut() else {
        return;
    };

    let player_translation = player_global.translation();
    let foot_plane_y = player_translation.y
        - (PLAYER_COLLIDER_HALF_HEIGHT + PLAYER_COLLIDER_RADIUS + BASE_CHARACTER_FOOT_OFFSET);
    let origin = ray.origin;
    let direction = Vec3::from(ray.direction);
    let direction_y = direction.y;

    if direction_y.abs() <= 1e-5 {
        return;
    }

    let t = (foot_plane_y - origin.y) / direction_y;
    if t <= 0.0 {
        return;
    }

    let hit_position = origin + direction * t;
    let mut look_direction = hit_position - player_translation;
    look_direction.y = 0.0;

    if look_direction.length_squared() <= 1e-4 {
        return;
    }

    let yaw = look_direction.x.atan2(look_direction.z);
    player_transform.rotation = Quat::from_rotation_y(yaw);
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

pub(crate) fn draw_player_gizmos(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<PlayerControlled>>,
) {
    for transform in &query {
        // 使用箭头展示默认朝向，便于确认模型前向与输入反馈一致。
        let origin = transform.translation;
        let forward = transform.forward();
        gizmos.arrow(origin, origin + forward * 2.0, Color::srgb_u8(255, 106, 60));

        // 在底部绘制水平圆圈，直观展示碰撞体半径位置。
        let base_center = origin
            - Vec3::Y
                * (PLAYER_COLLIDER_HALF_HEIGHT
                    + PLAYER_COLLIDER_RADIUS
                    + BASE_CHARACTER_FOOT_OFFSET);
        gizmos.circle(
            Isometry3d::from_translation(base_center)
                * Isometry3d::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            PLAYER_COLLIDER_RADIUS,
            Color::srgb_u8(120, 220, 255),
        );
    }
}
