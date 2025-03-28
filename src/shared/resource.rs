use bevy::prelude::*;
use bevy_renet::renet::ClientId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ResInitPlug;

impl Plugin for ResInitPlug {
    fn build(&self, app: &mut App) {
        app.init_resource::<Lobby>();
        app.init_resource::<MovementInput>();
    }
}

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<ClientId, Entity>,
}

#[derive(Component, Debug, Default, Serialize, Deserialize, Resource)]
pub struct MovementInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}
