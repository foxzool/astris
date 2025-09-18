use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

/// 标记跟随实体
#[derive(Component)]
pub struct FollowTarget;

/// 摄像机跟随配置
#[derive(Resource)]
pub struct CameraConfig {
    pub offset: Vec3,    // 玩家到摄像机的偏移
    pub smoothness: f32, // 插值平滑系数
    pub zoom_speed: f32, // 缩放速度
    pub min_zoom: f32,   // 最近距离
    pub max_zoom: f32,   // 最远距离
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            offset: Vec3::new(-10.0, 10.0, 10.0), // 斜 45° 偏移
            smoothness: 5.0,
            zoom_speed: 2.0,
            min_zoom: 5.0,
            max_zoom: 25.0,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraConfig::default())
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (follow_player, handle_zoom));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(-10.0, 10.0, 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("Main Camera"),
    ));
}

fn follow_player(
    time: Res<Time>,
    config: Res<CameraConfig>,
    player_query: Query<&Transform, With<FollowTarget>>,
    mut cam_query: Query<&mut Transform, (With<Camera>, Without<FollowTarget>)>,
) {
    if let Ok(player_tf) = player_query.single()
        && let Ok(mut cam_tf) = cam_query.single_mut()
    {
        // 目标位置 = 玩家位置 + 偏移
        let target_pos = player_tf.translation + config.offset;
        let current_pos = cam_tf.translation;

        // 平滑插值
        let new_pos = current_pos.lerp(target_pos, config.smoothness * time.delta_secs());
        cam_tf.translation = new_pos;

        // 始终看向玩家
        cam_tf.look_at(player_tf.translation, Vec3::Y);
    }
}

fn handle_zoom(mut scroll_events: EventReader<MouseWheel>, mut config: ResMut<CameraConfig>) {
    for event in scroll_events.read() {
        // 滚轮向上 = 拉近，向下 = 拉远
        let zoom_change = event.y * config.zoom_speed;

        // 计算当前与玩家的距离（取 offset 的长度）
        let current_len = config.offset.length();
        let new_len = (current_len - zoom_change).clamp(config.min_zoom, config.max_zoom);

        // 归一化方向 * 新长度
        config.offset = config.offset.normalize() * new_len;
    }
}
