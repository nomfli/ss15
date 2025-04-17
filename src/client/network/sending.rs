use crate::{
    client::render::chat::SendChatMsg,
    shared::{messages::ClientMessages, resource::MovementInput},
};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub(crate) struct ClientSendingPlug;

impl Plugin for ClientSendingPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, client_send_movement);
    }
}

pub(crate) fn client_send_movement(
    player_input: Res<MovementInput>,
    mut client: ResMut<RenetClient>,
) {
    if let Ok(input_message) = bincode::serialize(&ClientMessages::MovementInput {
        up: player_input.up,
        down: player_input.down,
        left: player_input.left,
        right: player_input.right,
    }) {
        client.send_message(DefaultChannel::Unreliable, input_message);
    }
}

pub(crate) fn client_send_chat(
    mut client: ResMut<RenetClient>,
    mut send_msg_ev: EventReader<SendChatMsg>,
) {
    for event in send_msg_ev.read() {
        if let Ok(chat_msg) = bincode::serialize(&ClientMessages::ChatMsg {
            mode: event.chat_mode,
            text: event.text.clone(),
        }) {
            client.send_message(DefaultChannel::Unreliable, chat_msg)
        }
    }
}
