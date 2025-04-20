use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(crate) struct ChatSharedPlug;

impl Plugin for ChatSharedPlug {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChatMode>();
    }
}

#[derive(Default, Debug, Resource, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum ChatMode {
    #[default]
    Say,
    Radio,
    Emotion,
    LOOC,
    OOC,
}

impl ChatMode {
    pub fn next(&mut self) -> ChatMode {
        match *self {
            ChatMode::Say => ChatMode::Radio,
            ChatMode::Radio => ChatMode::Emotion,
            ChatMode::Emotion => ChatMode::LOOC,
            ChatMode::LOOC => ChatMode::OOC,
            ChatMode::OOC => ChatMode::Say,
        }
    }
}
