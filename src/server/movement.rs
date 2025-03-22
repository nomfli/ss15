use crate::shared::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const MAX_MOVE_SPEED: f32 = 1000.0;
pub const ACCELERATION: f32 = 100.0;
pub const FRICTION: f32 = 0.9;

#[derive(Component, Debug, Default, Serialize, Deserialize)]
pub struct MaxSpeed(pub f32);

#[derive(Component, Debug, Default, Serialize, Deserialize)]
pub struct Acceleration(pub f32);

#[derive(Component, Debug, Default, Serialize, Deserialize)]
pub struct Speed {
    pub x: f32,
    pub y: f32,
}

pub fn move_players_system(mut query: Query<(&PlayerInput, &Acceleration, &mut Speed)>) {
    for (input, acceleration, mut speed) in query.iter_mut() {
        let mut dir = Vec2::new(0.0, 0.0);
        let acc_value = acceleration.0;
        if input.right {
            dir.x += 1.0;
        }
        if input.left {
            dir.x -= 1.0;
        }
        if input.up {
            dir.y += 1.0;
        }
        if input.down {
            dir.y -= 1.0;
        }

        if dir.length() > 0.0 {
            dir = dir.normalize();
            speed.x += dir.x * acc_value;
            speed.y += dir.y * acc_value;
        }
    }
}

pub fn velocity(time: Res<Time>, mut query: Query<(&mut Transform, &MaxSpeed, &mut Speed)>) {
    for (mut transform, max_speed, mut speed) in query.iter_mut() {
        let speed_vec = Vec2::new(speed.x, speed.y);
        let max_speed_value = max_speed.0;
        if speed_vec.length() > max_speed_value {
            let limited = speed_vec.normalize() * max_speed_value;
            speed.x = limited.x;
            speed.y = limited.y;
        }
        transform.translation.x += speed.x * time.delta_secs();
        transform.translation.y += speed.y * time.delta_secs();
        speed.x *= FRICTION;
        speed.y *= FRICTION;
        if speed.x.abs() < 0.1 {
            speed.x = 0.0;
        }
        if speed.y.abs() < 0.1 {
            speed.y = 0.0;
        }
    }
}
