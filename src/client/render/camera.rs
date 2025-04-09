use crate::shared::components::PlayerEntity;
use bevy::prelude::*;

pub struct CameraPlug;

impl Plugin for CameraPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_follow_smooth);
    }
}

fn camera_follow_smooth(
    player: Query<&Transform, With<PlayerEntity>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<PlayerEntity>)>,
    time: Res<Time>,
) {
    for mut camera_transform in camera.iter_mut() {
        let Ok(player_transform) = player.get_single() else {
            return;
        };
        let lerp_speed = 5.0;
        let target = player_transform.translation;
        camera_transform.translation = camera_transform
            .translation
            .lerp(target, lerp_speed * time.delta_secs());
        camera_transform.translation.z = 100.0;
    }
}
