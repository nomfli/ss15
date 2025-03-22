use crate::movement::*;
use crate::shared::hands::*;
use crate::shared::*;
use bevy::prelude::*;
use bevy_renet::netcode::*;
use bevy_renet::renet::*;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::SystemTime;

pub mod hands;
pub mod movement;

pub fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let public_addr = SERVER_ADDR.parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let server = RenetServer::new(ConnectionConfig::default());
    (server, transport)
}

pub fn server_send_movement(mut server: ResMut<RenetServer>, query: Query<(&Transform, &Player)>) {
    let cords_data: HashMap<ClientId, [f32; 2]> = HashMap::new();
    let mut players = ServerMessages::ChangeTransform { cords_data };

    for (transform, player) in query.iter() {
        if let ServerMessages::ChangeTransform { cords_data } = &mut players {
            let cords: [f32; 2] = transform.translation.truncate().into();
            cords_data.insert(player.id, cords);
        }
    }

    if let Ok(sync_message) = bincode::serialize(&players) {
        server.broadcast_message(DefaultChannel::Unreliable, sync_message);
    }
}

pub fn server_send_hands(
    mut server: ResMut<RenetServer>,
    query: Query<(&Player, &HandsCharacter, &IAmGrabbed)>,
) {
    let mut hands_char_data = ServerMessages::ChangeHands {
        hands_data: HashMap::new(),
    };
    for (player, hands_char, i_am_grabbed) in query.iter() {
        if let ServerMessages::ChangeHands { hands_data } = &mut hands_char_data {
            hands_data.insert(player.id, (hands_char.clone(), i_am_grabbed.clone()));
        }

        let hand_message = bincode::serialize(&hands_char_data).unwrap();
        server.broadcast_message(DefaultChannel::Unreliable, hand_message);
    }
}

pub fn update_server_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let player_entity_id = spawn_player_server(&mut commands, client_id);
                commands
                    .entity(player_entity_id)
                    .insert(IAmGrabbed(false))
                    .insert(HandsCharacter {
                        selected: 0,
                        hands: vec![
                            Hand {
                                grabbed_entity: None,
                            },
                            Hand {
                                grabbed_entity: None,
                            },
                        ],
                    })
                    .insert(SpriteName("red_sqr".to_string()));
                lobby.players.insert(*client_id, player_entity_id);

                for &player_id in lobby.players.keys() {
                    let message_about_old_connected =
                        bincode::serialize(&ServerMessages::PlayerConnected { id: player_id })
                            .unwrap();

                    server.send_message(
                        *client_id,
                        DefaultChannel::ReliableOrdered,
                        message_about_old_connected,
                    );
                    if player_id != *client_id {
                        let message_about_new_connected =
                            bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id });
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

    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let player_input: PlayerInput = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                commands.entity(*player_entity).insert(player_input);
            }
        }
    }
}

pub fn spawn_player_server(commands: &mut Commands, client_id: &u64) -> Entity {
    let ent = commands
        .spawn(Player { id: *client_id })
        .insert(Acceleration(ACCELERATION))
        .insert(Transform {
            ..Default::default()
        })
        .insert(MaxSpeed(MAX_MOVE_SPEED))
        .insert(Speed { x: 0.0, y: 0.0 })
        .id();
    ent
}
