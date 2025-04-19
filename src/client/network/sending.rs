use crate::{
    client::render::hands::TryToGrabbEvent,
    shared::{messages::ClientMessages, resource::MovementInput},
};

use bevy::prelude::*;
use bevy_renet::renet::*;

pub(crate) struct ClientSendingPlug;

impl Plugin for ClientSendingPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, client_send_movement);
        app.add_systems(Update, send_grabbing);

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


pub(crate) fn send_grabbing(
    mut reader: EventReader<TryToGrabbEvent>,
    mut client: ResMut<RenetClient>,
) {
    for event in reader.read() {
        if let Ok(grabb_msg) = bincode::serialize(&ClientMessages::Grab {
            can_be_grabbed: event.can_be_grabbed,
            hand_idx: event.hand_idx,
        }) {
            client.send_message(DefaultChannel::Unreliable, grabb_msg);
        }
    }
}

