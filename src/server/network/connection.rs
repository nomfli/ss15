use crate::server::logic::movement::*;

use crate::shared::{
    components::{Hand, Hands, Player},
    messages::ServerMessages,
    resource::Lobby,
};

use bevy::prelude::*;
use bevy_renet::renet::*;

pub struct ConnectionHandlerPlug;

impl Plugin for ConnectionHandlerPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, client_connection_handler);
        app.add_event::<SendPlayerConnection>();
    }
}

#[derive(Event)]
pub(crate) struct SendPlayerConnection {
    pub client_id: u64,
}

pub(crate) fn client_connection_handler(
    mut client_connected: EventReader<SendPlayerConnection>,
    mut lobby: ResMut<Lobby>,
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
) {
    for event in client_connected.read() {
        let client_id = &event.client_id;
        let player_entity_id = spawn_player_server(&mut commands, client_id);
        lobby.players.insert(*client_id, player_entity_id);

        for &player_id in lobby.players.keys() {
            let Some(server_ent) = lobby.players.get(&player_id) else {
                continue;
            };
            let connected_users_msg = bincode::serialize(&ServerMessages::PlayerConnected {
                client_id: player_id,
                ent_id: *server_ent,
            })
            .unwrap();
            server.send_message(
                *client_id,
                DefaultChannel::ReliableOrdered,
                connected_users_msg,
            );
            if player_id != *client_id {
                let Some(server_ent) = lobby.players.get(client_id) else {
                    continue;
                };
                let new_connected_msg = bincode::serialize(&ServerMessages::PlayerConnected {
                    client_id: *client_id,
                    ent_id: *server_ent,
                });
                if let Ok(msg) = new_connected_msg {
                    server.send_message(player_id, DefaultChannel::ReliableOrdered, msg);
                }
            }
        }
    }
}

pub(crate) fn spawn_player_server(commands: &mut Commands, client_id: &u64) -> Entity {
    let ent = commands
        .spawn(Sprite {
            color: Color::srgb_u8(255, 0, 0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        })
        .insert(Player { id: *client_id })
        .insert(Hands {
            all_hands: vec![
                Hand {
                    grab_ent: None,
                    hand_len: 100000.0,
                },
                Hand {
                    grab_ent: None,
                    hand_len: 100000.0,
                },
            ],
            selected_hand: 0,
        })
        .insert(Acceleration(ACCELERATION))
        .insert(MaxSpeed(MAX_MOVE_SPEED))
        .insert(Speed { x: 0.0, y: 0.0 })
        .id();
    ent
}
