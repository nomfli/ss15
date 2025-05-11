use crate::{
    server::logic::collision::check_collisions,
    shared::{
        components::{Player, Speed},
        messages::ServerMessages,
        resource::MovementInput,
    },
};

use bevy::prelude::*;
use bevy_renet::renet::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

pub(crate) const MAX_MOVE_SPEED: f32 = 1000.0;
pub(crate) const ACCELERATION: f32 = 100.0;

#[derive(Resource, Debug, Default, Serialize, Deserialize)]
pub(crate) struct Positions(pub HashMap<Entity, [f32; 2]>);

#[derive(Component, Debug, Default, Serialize, Deserialize)]
pub(crate) struct MaxSpeed(pub f32);

#[derive(Component, Debug, Default, Serialize, Deserialize)]
pub(crate) struct Acceleration(pub f32);

pub(crate) struct MovementServerPlug;

impl Plugin for MovementServerPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_players_system.after(check_collisions));
        app.add_systems(Update, velocity);
        app.add_systems(Update, server_sync_players_movement);
        app.init_resource::<Positions>();
    }
}

pub(crate) fn move_players_system(mut query: Query<(&MovementInput, &Acceleration, &mut Speed)>) {
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

pub(crate) fn velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MaxSpeed, &mut Speed, Entity)>,
) {
    for (mut transform, max_speed, mut speed, ent) in query.iter_mut() {
        let speed_vec = Vec2::new(speed.x, speed.y);
        let max_speed_value = max_speed.0;
        if speed_vec.length() > max_speed_value {
            let limited = speed_vec.normalize() * max_speed_value;
            speed.x = limited.x;
            speed.y = limited.y;
        }
        transform.translation.x += speed.x * time.delta_secs();
        transform.translation.y += speed.y * time.delta_secs();
        speed.x *= 0.95;
        speed.y *= 0.95;
        println!("{:?}", (transform.translation, speed.clone(), ent));
        if speed.x.abs() < 0.1 {
            speed.x = 0.0;
        }
        if speed.y.abs() < 0.1 {
            speed.y = 0.0;
        }
    }
}

pub(crate) fn server_sync_players_movement(
    mut server: ResMut<RenetServer>,
    query: Query<(&Transform, Entity)>,
    mut positions: ResMut<Positions>,
) {
    for (transform, ent) in query.iter() {
        positions
            .0
            .insert(ent, transform.translation.truncate().into());
    }
    if let Ok(sync_message) =
        bincode::serialize(&ServerMessages::SendPositions(positions.0.clone()))
    {
        server.broadcast_message(DefaultChannel::Unreliable, sync_message);
    }
}

pub(crate) trait Id {
    fn id(&self) -> u64;
}

impl Id for Player {
    fn id(&self) -> u64 {
        self.id
    }
}
