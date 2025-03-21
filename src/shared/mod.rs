use crate::shared::hands::*;
use bevy::prelude::*;
use bevy_renet::renet::ClientId;
use serde::*;
use std::collections::HashMap;

pub mod hands;

pub const SERVER_ADDR: &str = "127.0.0.1:5000";
pub const PROTOCOL_ID: u64 = 1;
#[derive(Component, Debug, Default, Serialize, Deserialize, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub throw: bool,
    pub left_mouse: bool,
    pub cursor_pos: Option<Vec2>,
    pub change_hand: bool,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected {
        id: ClientId,
    },
    PlayerDisconnected {
        id: ClientId,
    },
    ChangeTransform {
        cords_data: HashMap<ClientId, [f32; 2]>,
    },
    ChangeHands {
        hands_data: HashMap<ClientId, (HandsCharacter, IAmGrabbed)>,
    },
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct Player {
    pub id: ClientId,
}

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<ClientId, Entity>,
}

#[derive(Debug, Default, Resource)]
pub struct Data {
    pub sprite: HashMap<String, Sprite>,
}
