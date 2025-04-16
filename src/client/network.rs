use crate::client::{connection::PlayerConnected, movement::ChangePositions};
use crate::shared::{
    components::Grabbable,
    messages::ServerMessages,
    resource::Lobby,
    sprites::{SpriteName, Sprites},
};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub struct ClientSyncPlayersPlug;
impl Plugin for ClientSyncPlayersPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, receive_message);
    }
}

pub(crate) fn receive_message(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    sprites: Res<Sprites>,
    mut change_pos_ev: EventWriter<ChangePositions>,
    mut user_connected_ev: EventWriter<PlayerConnected>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { client_id, ent_id } => {
                user_connected_ev.send(PlayerConnected { client_id, ent_id });
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
                change_pos_ev.send(ChangePositions(players));
            }

            Ok(ServerMessages::AddItem(item)) => {
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
