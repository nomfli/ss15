

use crate::client::render::{
    connection::PlayerConnected,
    hands::ShouldGrab,
    movement::{ChangePositions, SpeedEvent},

};
use crate::shared::{
    components::Grabbable,
    events::ThrowAnswerEvent,
    messages::ServerMessages,
    resource::{Entities, Lobby},
    sprites::{SpriteName, Sprites},
};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub struct ClientNetworkPlug;

impl Plugin for ClientNetworkPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, receive_message);
    }
}

pub(crate) fn receive_message(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    mut ents: ResMut<Entities>,
    sprites: Res<Sprites>,
    (mut change_pos_ev, mut user_connected_ev, mut grab_event, mut speed_event, mut throw_event): (
        EventWriter<ChangePositions>,
        EventWriter<PlayerConnected>,
        EventWriter<ShouldGrab>,
        EventWriter<SpeedEvent>,
        EventWriter<ThrowAnswerEvent>,
    ),
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
                //need to
                //incapsulate
                let ([x, y], name, ent, grabbable) = item;
                let Some(sprite) = sprites.0.get(&name.0) else {
                    continue;
                };
                let client_ent_id = commands
                    .spawn(Transform {
                        translation: Vec3::new(x, y, 0.0),
                        ..Default::default()
                    })
                    .insert(SpriteName(name.0))
                    .insert(Grabbable(grabbable.0))
                    .insert(sprite.clone())
                    .id();
                ents.entities.insert(client_ent_id, ent);
            }
            Ok(ServerMessages::GrabAnswer(ent, id)) => {
                grab_event.send(ShouldGrab {
                    i_must_be_grabbed: ent,
                    who_should_grab: id,
                });
            }
            Ok(ServerMessages::Speed(speed)) => {
                speed_event.send(SpeedEvent(speed));
            Ok(ServerMessages::ThrowAnswer {
                client_id,
                hand_idx,
                where_throw,
            }) => {
                throw_event.send(ThrowAnswerEvent {
                    client: client_id,
                    hand_idx,
                    where_throw,
                });
            }
            _ => {}
        }
    }
}
