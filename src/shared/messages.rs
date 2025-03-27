use crate::shared::{components::Grabbable, sprites::SpriteName};
use bevy::prelude::*;
use bevy_renet::renet::ClientId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { client_id: ClientId, ent_id: Entity },
    PlayerDisconnected { id: ClientId },
    SendPositions(HashMap<ClientId, [f32; 2]>),
    SendItem(([f32; 2], SpriteName, Entity, Grabbable)),
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessages {
    MovementInput {
        up: bool,
        left: bool,
        right: bool,
        down: bool,
    },
}
