use crate::shared::{chat::ChatMode, measure};
use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub(crate) struct MsgHandlerEvent {
    pub mode: ChatMode,
    pub text: String,
    pub client_ent: Entity,
}

pub(crate) fn handle_chat_msg(
    mut handle_msg_ev: EventReader<MsgHandlerEvent>,
    query: Query<&Transform>,
) {
    for event in handle_msg_ev.read() {
        let transform = query.get(event.client_ent);
        match event.mode {
            ChatMode::Say => {let clients = }
            _ => {} //TODO
        }
    }
}
