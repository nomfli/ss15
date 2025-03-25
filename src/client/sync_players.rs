use crate::client::hands::ShouldGrabb;
use crate::shared::{
    components::{Hand, Hands, PlayerEntity, ServerEntityId},
    messages::ServerMessages,
    resource::Lobby,
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

pub fn client_sync_players(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    client_transport: Res<NetcodeClientTransport>,
    mut lobby: ResMut<Lobby>,
    mut grab_event: EventWriter<ShouldGrabb>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { client_id, ent_id } => {
                let this_client_id = client_transport.client_id();
                let player_entity_id = spawn_player_client(&mut commands, ent_id);

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
            Ok(ServerMessages::GrabAnswer(ent, id)) => {
                grab_event.send(ShouldGrabb {
                    i_must_be_grabbed: ent,
                    who_should_grabe: id,
                });
            }
            _ => {}
        }
    }
}

fn spawn_player_client(commands: &mut Commands, ent_id: Entity) -> Entity {
    let player_entity_id = commands
        .spawn(Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            color: Color::srgb(255.0, 0.0, 0.0),
            ..Default::default()
        })
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
        .insert(ServerEntityId { ent_id })
        .id();
    player_entity_id
}
