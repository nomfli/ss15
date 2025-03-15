use crate::shared::*;
use bevy::prelude::*;
use bevy_renet::netcode::*;
use bevy_renet::renet::*;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::SystemTime;

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

pub fn move_players_system(
    mut query: Query<(
        &mut Transform,
        &PlayerInput,
        &MaxSpeed,
        &Acceleration,
        &mut Speed,
    )>,
    time: Res<Time>,
) {
    for (mut transform, input, max_speed, acceleration, mut speed) in query.iter_mut() {
        let mut dir = Vec2::new(0.0, 0.0);
        let max_speed_value = max_speed.0;
        let acc_value = acceleration.0;
        if input.right {
            dir.x += 1.0;
        }
        if input.left {
            dir.x -= 1.0;
        }
        if input.up {
            dir.y += 1.0;
        }
        if input.down {
            dir.y -= 1.0;
        }

        if dir.length() > 0.0 {
            dir = dir.normalize();
            speed.x += dir.x * acc_value;
            speed.y += dir.y * acc_value;
        }

        let speed_vec = Vec2::new(speed.x, speed.y);
        if speed_vec.length() > max_speed_value {
            let limited = speed_vec.normalize() * max_speed_value;
            speed.x = limited.x;
            speed.y = limited.y;
        }
        transform.translation.x += speed.x * time.delta_secs();
        transform.translation.y += speed.y * time.delta_secs();
        speed.x *= 0.95;
        speed.y *= 0.95;
        if speed.x.abs() < 0.1 {
            speed.x = 0.0;
        }
        if speed.y.abs() < 0.1 {
            speed.y = 0.0;
        }
    }
}

pub fn server_sync_players(mut server: ResMut<RenetServer>, query: Query<(&Transform, &Player)>) {
    let mut players: HashMap<ClientId, [f32; 3]> = HashMap::new();
    for (transform, player) in query.iter() {
        players.insert(player.id, transform.translation.into());
    }
    let sync_message = bincode::serialize(&players).unwrap();
    server.broadcast_message(DefaultChannel::Unreliable, sync_message);
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
                let player_entity_id = commands
                    .spawn(Sprite {
                        color: Color::srgb(255.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(100.0, 100.0)),
                        ..Default::default()
                    })
                    .insert(PlayerInput::default())
                    .insert(Player { id: *client_id })
                    .insert(Acceleration(ACCELERATION))
                    .insert(MaxSpeed(MAX_MOVE_SPEED))
                    .insert(Speed { x: 0.0, y: 0.0 })
                    .id();
                lobby.players.insert(*client_id, player_entity_id);

                for &player_id in lobby.players.keys() {
                    let message =
                        bincode::serialize(&ServerMessages::PlayerConnected { id: player_id })
                            .unwrap();
                    server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
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
