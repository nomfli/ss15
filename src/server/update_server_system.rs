use crate::server::connection::*;
use crate::server::hands::*;
use crate::shared::{
    messages::*,
    resource::{Lobby, MovementInput},
};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub struct UpdateServerPlug;

impl Plugin for UpdateServerPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, connections_handler);
        app.add_systems(Update, message_handler);
    }
}

pub fn connections_handler(
    mut server_events: EventReader<ServerEvent>,
    mut client_connected: EventWriter<SendPlayerConnection>,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                client_connected.send(SendPlayerConnection {
                    client_id: *client_id,
                });
            }

            ServerEvent::ClientDisconnected {
                client_id,
                reason: _,
            } => {
                if let Some(player_entity) = lobby.players.remove(client_id) {
                    commands.entity(player_entity).despawn();
                }
                lobby.players.remove(client_id);
                let message =
                    bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
        }
    }
}

fn message_handler(
    mut commands: Commands,
    lobby: Res<Lobby>,
    mut server: ResMut<RenetServer>,
    mut grap_ev: EventWriter<GrabEvent>,
    mut throw_ev: EventWriter<ServerThrowEvent>,
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::Unreliable) {
            let client_msg = bincode::deserialize(&message);
            match client_msg {
                Ok(ClientMessages::MovementInput {
                    up,
                    left,
                    right,
                    down,
                }) => {
                    let Some(ent) = lobby.players.get(&client_id) else {
                        continue;
                    };
                    commands.entity(*ent).insert(MovementInput {
                        up,
                        down,
                        left,
                        right,
                    });
                }
                Ok(ClientMessages::Grab {
                    can_be_grabbed,
                    hand_idx,
                }) => {
                    let Some(i_want_grabb) = lobby.players.get(&client_id) else {
                        continue;
                    };
                    grap_ev.send(GrabEvent {
                        i_want_grabb: *i_want_grabb,
                        can_be_grabbed,
                        hand_idx,
                        client: client_id,
                    });
                }
                Ok(ClientMessages::Drop {
                    hand_idx,
                    where_throw,
                }) => {
                    let Some(i_want_throw) = lobby.players.get(&client_id) else {
                        continue;
                    };
                    throw_ev.send(ServerThrowEvent {
                        i_want_throw: *i_want_throw,
                        client: client_id,
                        hand_idx,
                        where_throw,
                    });
                }

                Err(_) => {}
            }
        }
    }
}
