use crate::shared::components::PlayerEntity;
use bevy::prelude::*;

pub(crate) fn camera_follow_smooth(
    player: Query<&Transform, With<PlayerEntity>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<PlayerEntity>)>,
) {
    for mut camera_transform in camera.iter_mut() {
        let Ok(player_transform) = player.get_single() else {
            return;
        };
        camera_transform.translation = player_transform.translation;
    }
}
