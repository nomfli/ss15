use crate::shared::{components::Direction, messages::ServerMessages, resource::Lobby};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub(crate) struct RotServerPlug;

impl Plugin for RotServerPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<DirectionEvent>();
        app.add_systems(Update, send_rotation);
    }
}

#[derive(Event)]
pub(crate) struct DirectionEvent {
    pub client: ClientId,
    pub direction: Direction,
}

pub(crate) fn send_rotation(
    mut rot_ev: EventReader<DirectionEvent>,
    mut server: ResMut<RenetServer>,
    lobby: Res<Lobby>,
) {
    for event in rot_ev.read() {
        for player in lobby.players.keys() {
            if *player != event.client {
                let Some(ent) = lobby.players.get(&event.client) else {
                    continue;
                };
                let Ok(rot_msg) =
                    bincode::serialize(&ServerMessages::Direction(event.direction, *ent))
                else {
                    continue;
                };
                server.send_message(*player, DefaultChannel::Unreliable, rot_msg);
            }
        }
    }
}
