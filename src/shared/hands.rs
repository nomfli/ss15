use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Default, Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct SpriteName(pub String);

#[derive(Component, Default, Debug, Clone, Serialize, Deserialize, Resource)]
pub struct HandsCharacter {
    pub hands: Vec<Hand>,
    pub selected: usize,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, Copy)]
pub struct IAmGrabbed(pub bool);

#[derive(Component, Default, Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Hand {
    pub grabbed_entity: Option<Entity>,
}
