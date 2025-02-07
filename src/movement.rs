use bevy::prelude::*;

#[derive(Component)]
pub struct Speed {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct MaxSpeed(pub f32);

#[derive(Component)]
pub struct Acceleration(pub f32);

pub fn keyboard_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut speed_query: Query<&mut Speed>,
    max_speed_query: Query<&MaxSpeed>,
    acceleration_query: Query<&Acceleration>,
) {
    for mut speed in speed_query.iter_mut() {
        let max_speed_value = max_speed_query.single().0;
        let acc_value = acceleration_query.single().0;

        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            speed.x += direction.x * acc_value;
            speed.y += direction.y * acc_value;
        }

        let speed_vec = Vec2::new(speed.x, speed.y);
        if speed_vec.length() > max_speed_value {
            let limited = speed_vec.normalize() * max_speed_value;
            speed.x = limited.x;
            speed.y = limited.y;
        }
    }
}

pub fn velocity(time: Res<Time>, mut query: Query<(&mut Transform, &mut Speed)>) {
    for (mut transform, mut speed) in query.iter_mut() {
        transform.translation.x += speed.x * time.delta_secs();
        transform.translation.y += speed.y * time.delta_secs();

        speed.x *= 0.95;
        speed.y *= 0.95;

        if speed.x.abs() < 0.1 {
            speed.x = 0.0;
        }
        if speed.y.abs() < 0.1 {
            speed.y = 0.0;
        }
    }
}
