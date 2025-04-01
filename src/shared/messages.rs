use crate::shared::{components::Grabbable, sprites::SpriteName};
use bevy::prelude::*;
use bevy_renet::renet::ClientId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { client_id: ClientId, ent_id: Entity },
    PlayerDisconnected { id: ClientId },
    GrabAnswer(Entity, ClientId),
    ThrowAnswer(Entity, ClientId, [f32; 2]),
    SendPositions(HashMap<ClientId, [f32; 2]>),
    AddItem(([f32; 2], SpriteName, Entity, Grabbable)),
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessages {
    MovementInput {
        up: bool,
        left: bool,
        right: bool,
        down: bool,
    },
    Grab {
        can_be_grabbed: Entity,
        hand_idx: usize,
    },
    Drop {
        hand_idx: usize,
        where_throw: Vec2,
    },
}
