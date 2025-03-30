use crate::shared::{
    components::{Grabbable, PlayerEntity},
    messages::ServerMessages,
    resource::Lobby,
    sprites::{SpriteName, Sprites},
};
use bevy::prelude::*;
use bevy_renet::netcode::NetcodeClientTransport;
use bevy_renet::renet::*;
pub struct ClientSyncPlayersPlug;

impl Plugin for ClientSyncPlayersPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, client_sync_players);
    }
}

pub(crate) fn client_sync_players(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    client_transport: Res<NetcodeClientTransport>,
    mut lobby: ResMut<Lobby>,
    sprites: Res<Sprites>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { client_id, ent_id } => {
                let this_client_id = client_transport.client_id();
                let player_entity_id = spawn_player_client(&mut commands, ent_id, &sprites);

                if this_client_id == client_id {
                    commands.entity(player_entity_id).insert(PlayerEntity);
                }
                lobby.players.insert(client_id, player_entity_id);
            }

            ServerMessages::PlayerDisconnected { id } => {
                if let Some(player_entity) = lobby.players.remove(&id) {
                    commands.entity(player_entity).despawn();
                }
            }
            _ => {}
        }
    }
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        match bincode::deserialize(&message) {
            Ok(ServerMessages::SendPositions(players)) => {
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
            Ok(ServerMessages::SendItem(item)) => {
                let ([x, y], name, _ent, grabbable) = item;
                let Some(sprite) = sprites.0.get(&name.0) else {
                    continue;
                };
                commands
                    .spawn(Transform {
                        translation: Vec3::new(x, y, 0.0),
                        ..Default::default()
                    })
                    .insert(SpriteName(name.0))
                    .insert(Grabbable(grabbable.0))
                    .insert(sprite.clone());
            }
            _ => {}
        }
    }
}

fn spawn_player_client(commands: &mut Commands, _ent_id: Entity, sprites: &Res<Sprites>) -> Entity {
    if let Some(sprite) = sprites.0.get("red_sqr") {
        let player_entity_id = commands
            .spawn(SpriteName("red_sqr".to_string()))
            .insert(sprite.clone())
            .id();

        player_entity_id
    } else {
        panic!("BAD DATA!"); //idk what entity i need to return here
    }
}
