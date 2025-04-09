use crate::shared::{
    components::{Grabbable, Speed},
    sprites::SpriteName,
};
use bevy::prelude::*;
use bevy_renet::renet::ClientId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { client_id: ClientId, ent_id: Entity },
    PlayerDisconnected { id: ClientId },
    SendPositions(HashMap<ClientId, [f32; 2]>),
    AddItem(([f32; 2], SpriteName, Entity, Grabbable)),
    GrabAnswer(Entity, ClientId),
    Speed(Speed),
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
}
