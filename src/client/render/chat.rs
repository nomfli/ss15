use bevy::prelude::*;

#[derive(Default, Debug, Resource)]
pub(crate) enum ChatMode {
    Say,
    Radio,
    Emotion,
    LOOC,
    OOC,
}

impl ChatMode {
    fn next(&self) {
        match *self {
            ChatMode::Say => ChatMode::Radio,
            ChatMode::OOC => ChatMode::Say,
            ChatMode::LOOC => ChatMode::Emotion,
            ChatMode::Radio => ChatMode::LOOC,
            ChatMode::Emotion => ChatMode::OOC,
        }
    }
}

pub(crate) fn change_chat_mode(mut chat_mode: ResMut<ChatMode>) {}
