use crate::server::movement::*;
use crate::shared::{
    components::{Grabbable, Player},
    messages::ServerMessages,
    resource::Lobby,
    sprites::SpriteName,
};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub struct ConnectionHandlerPlug;

impl Plugin for ConnectionHandlerPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, client_connection_handler);
        app.add_event::<SendPlayerConnection>();
        app.add_event::<SendItems>();
        app.add_systems(Update, send_items);
    }
}

#[derive(Event)]
pub struct SendPlayerConnection {
    pub client_id: u64,
}

#[derive(Event)]
pub struct SendItems {
    pub client_id: u64,
}

pub fn client_connection_handler(
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
                if let Ok(msg) = message_about_new_connected {
                    server.send_message(player_id, DefaultChannel::ReliableOrdered, msg);
                }
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
        .id();
    ent
}

pub fn send_items(
    mut send_item: EventReader<SendItems>,
    items: Query<(&Transform, &SpriteName, Entity, &Grabbable), Without<Player>>,
    mut server: ResMut<RenetServer>,
) {
    for event in send_item.read() {
        for item in items.iter() {
            let (trans, name, ent, grabbable) = item;
            let Vec2 { x, y } = trans.translation.truncate();
            let item_msg = bincode::serialize(&ServerMessages::SendItem((
                [x, y],
                name.clone(),
                ent,
                *grabbable,
            )));

            if let Ok(msg) = item_msg {
                server.send_message(event.client_id, DefaultChannel::Unreliable, msg)
            }
        }
    }
}
