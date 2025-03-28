use bevy::prelude::*;
use bevy_renet::renet::ClientId;
use serde::{Deserialize, Serialize};

pub const SERVER_ADDR: &str = "127.0.0.1:5000";
pub const PROTOCOL_ID: u64 = 1;

#[derive(Clone, Copy, Default, Debug, Component)]
pub struct PlayerEntity;

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct Player {
    pub id: ClientId,
}

#[derive(Clone, Copy, Default, Debug, Component, Serialize, Deserialize)]
pub struct Grabbable(pub bool);
