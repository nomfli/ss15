use bevy::prelude::*;
use bevy_renet::renet::ClientId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;


pub struct ResInitPlug;

impl Plugin for ResInitPlug {
    fn build(&self, app: &mut App) {
        app.init_resource::<Lobby>();
        app.init_resource::<MovementInput>();
        app.insert_resource(Entities {
            entities: Bimap::new(),
        });

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

#[derive(Debug, Default)]
pub struct Bimap<T, V> {
    to_second: HashMap<T, V>,
    to_first: HashMap<V, T>,
}

impl<T, V> Bimap<T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Bimap {
            to_second: HashMap::new(),
            to_first: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: T, value: V) {
        self.to_second.insert(key.clone(), value.clone());
        self.to_first.insert(value, key);
    }

    pub fn get_by_first(&self, key: &T) -> Option<&V> {
        self.to_second.get(key)
    }

    pub fn get_by_second(&self, value: &V) -> Option<&T> {
        self.to_first.get(value)
    }
}

#[derive(Debug, Resource)]
pub(crate) struct Entities {
    pub entities: Bimap<Entity, Entity>, //from client to server
}

