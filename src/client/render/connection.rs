use crate::shared::{

    components::{Hand, Hands, PlayerEntity},
    resource::{Entities, Lobby},

    sprites::{SpriteName, Sprites},
};
use bevy::prelude::*;
use bevy_renet::netcode::NetcodeClientTransport;

pub struct ConnectionPlug;

impl Plugin for ConnectionPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_connected);
        app.add_event::<PlayerConnected>();
    }
}

#[derive(Debug, Event, Copy, Clone)]
pub(crate) struct PlayerConnected {
    pub client_id: u64,
    pub ent_id: Entity,
}

pub(crate) fn player_connected(
    mut player_connected_ev: EventReader<PlayerConnected>,
    mut lobby: ResMut<Lobby>,
    client_transport: Res<NetcodeClientTransport>,
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut ents: ResMut<Entities>,

) {
    for event in player_connected_ev.read() {
        let client_id = event.client_id;
        let ent_id = event.ent_id;
        let this_client_id = client_transport.client_id();
        let player_entity_id = spawn_player_client(&mut commands, ent_id, &sprites);
        if this_client_id == client_id {
            commands.entity(player_entity_id).insert(PlayerEntity);
        }
        lobby.players.insert(client_id, player_entity_id);
        ents.entities.insert(player_entity_id, event.ent_id);

    }
}

fn spawn_player_client(commands: &mut Commands, ent_id: Entity, sprites: &Res<Sprites>) -> Entity {
    if let Some(sprite) = sprites.0.get("red_sqr") {
        let player_entity_id = commands
            .spawn(SpriteName("red_sqr".to_string()))
            .insert(sprite.clone())
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

        player_entity_id
    } else {
        panic!("Missing sprite 'red_sqr' for entity ID {:?}", ent_id);
    }
}
