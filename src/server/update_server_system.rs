use crate::server::hands::*;
use crate::server::movement::*;
use crate::shared::{
    components::{Hand, Hands, Player},
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
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let player_entity_id = spawn_player_server(&mut commands, client_id);
                lobby.players.insert(*client_id, player_entity_id);

                for &player_id in lobby.players.keys() {
                    let Some(server_ent) = lobby.players.get(&player_id) else {
                        continue;
                    };
                    let message_about_old_connected =
                        bincode::serialize(&ServerMessages::PlayerConnected {
                            client_id: player_id,
                            ent_id: *server_ent,
                        })
                        .unwrap();

                    server.send_message(
                        *client_id,
                        DefaultChannel::ReliableOrdered,
                        message_about_old_connected,
                    );
                    if player_id != *client_id {
                        let Some(server_ent) = lobby.players.get(client_id) else {
                            continue;
                        };
                        let message_about_new_connected =
                            bincode::serialize(&ServerMessages::PlayerConnected {
                                client_id: *client_id,
                                ent_id: *server_ent,
                            });
                        match message_about_new_connected {
                            Ok(msg) => {
                                server.send_message(
                                    player_id,
                                    DefaultChannel::ReliableOrdered,
                                    msg,
                                );
                            }
                            Err(_) => {}
                        }
                    }
                }
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

                Err(_) => {}
            }
        }
    }
}

pub fn spawn_player_server(commands: &mut Commands, client_id: &u64) -> Entity {
    let ent = commands
        .spawn(Sprite {
            color: Color::srgb(255.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        })
        .insert(Player { id: *client_id })
        .insert(Acceleration(ACCELERATION))
        .insert(MaxSpeed(MAX_MOVE_SPEED))
        .insert(Speed { x: 0.0, y: 0.0 })
        .insert(Hands {
            all_hands: vec![
                Hand {
                    grabb_ent: None,
                    hand_len: 100000.0,
                },
                Hand {
                    grabb_ent: None,
                    hand_len: 100000.0,
                },
            ],
            selected_hand: 0,
        })
        .id();
    ent
}
