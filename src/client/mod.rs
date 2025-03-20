use crate::shared::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_renet::netcode::*;
use bevy_renet::renet::*;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::SystemTime;

pub mod init;

pub fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let server_addr = SERVER_ADDR.parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = rand::random::<u64>();
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(ConnectionConfig::default());

    (client, transport)
}

pub fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut player_input: ResMut<PlayerInput>,
) {
    player_input.left =
        keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    player_input.right =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
    player_input.up =
        keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    player_input.down =
        keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);

    player_input.throw = keyboard_input.pressed(KeyCode::KeyQ);

    player_input.left_mouse = mouse_button.pressed(MouseButton::Left);
    if keyboard_input.just_pressed(KeyCode::KeyX) {
        player_input.change_hand = true;
    } else {
        player_input.change_hand = false;
    }
    let window = q_windows.single();
    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok((camera, camera_transform)) = q_camera.get_single() {
            match camera.viewport_to_world(camera_transform, cursor_pos) {
                Ok(ray) => {
                    player_input.cursor_pos = Some(ray.origin.truncate());
                }
                Err(_) => {
                    player_input.cursor_pos = None;
                }
            }
        }
    } else {
        player_input.cursor_pos = None;
    }
}

pub fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
    let input_message = bincode::serialize(&*player_input).unwrap();

    client.send_message(DefaultChannel::ReliableOrdered, input_message);
}

pub fn client_sync_players(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    data: Res<Data>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { id } => match data.sprite.get("red_sqr") {
                Some(red_sqr) => {
                    let player_entity_id = commands.spawn(red_sqr.clone()).id();
                    lobby.players.insert(id, player_entity_id);
                }
                _ => {}
            },

            ServerMessages::PlayerDisconnected { id } => {
                if let Some(player_entity) = lobby.players.remove(&id) {
                    commands.entity(player_entity).despawn();
                }
            }
        }
    }
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let players: HashMap<ClientId, [f32; 2]> = bincode::deserialize(&message).unwrap();
        for (player_id, transition) in players.iter() {
            if let Some(player_entity) = lobby.players.get(player_id) {
                let [x, y] = *transition;
                let transform = Transform {
                    translation: Vec3::new(x, y, 0.0),

                    ..Default::default()
                };
                commands.entity(*player_entity).insert(transform);
            }
        }
    }
}
