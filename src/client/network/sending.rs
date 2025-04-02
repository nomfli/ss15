use crate::shared::{messages::ClientMessages, resource::MovementInput};
use bevy::prelude::*;
use bevy_renet::renet::*;

struct ClientSendingPlug;

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
