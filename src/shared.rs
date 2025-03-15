use bevy::prelude::*;
use bevy_renet::renet::ClientId;
use serde::*;
use std::collections::HashMap;
pub const SERVER_ADDR: &str = "127.0.0.1:5000";
pub const PROTOCOL_ID: u64 = 1;
pub const MAX_MOVE_SPEED: f32 = 1000.0;
pub const ACCELERATION: f32 = 100.0;
#[derive(Component, Debug, Default, Serialize, Deserialize, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Component, Debug, Default, Serialize, Deserialize)]
pub struct Speed {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Default, Serialize, Deserialize)]
pub struct MaxSpeed(pub f32);

#[derive(Component, Debug, Default, Serialize, Deserialize)]
pub struct Acceleration(pub f32);

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: ClientId },
    PlayerDisconnected { id: ClientId },
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct Player {
    pub id: ClientId,
}

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<ClientId, Entity>,
}

pub fn startup(mut commands: Commands) {
    commands.spawn(Camera2d { ..default() });
}
