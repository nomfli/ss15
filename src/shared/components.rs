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

#[derive(Component, Clone, Debug)]
pub struct ServerEntityId {
    pub ent_id: Entity,
}

#[derive(Default, Copy, Clone, Debug, Component, Serialize, Deserialize)]
pub struct Hand {
    pub grabb_ent: Option<Entity>,
    pub hand_len: f32,
}

#[derive(Clone, Default, Debug, Component)]
pub struct Hands {
    pub all_hands: Vec<Hand>,
    pub selected_hand: usize,
}
