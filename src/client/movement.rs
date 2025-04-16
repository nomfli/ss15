use crate::shared::{
    messages::ClientMessages,
    resource::{Lobby, MovementInput},
};
use bevy::prelude::*;
use bevy_renet::renet::*;
use std::collections::HashMap;

pub struct MovementClientPlug;

impl Plugin for MovementClientPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangePositions>();
        app.add_systems(Update, client_send_movement);
        app.add_systems(Update, change_position);
    }
}

#[derive(Default, Debug, Clone, Event)]
pub(crate) struct ChangePositions(pub HashMap<u64, [f32; 2]>);

pub(crate) fn client_send_movement(
    player_input: Res<MovementInput>,
    mut client: ResMut<RenetClient>,
) {
    if let Ok(input_message) = bincode::serialize(&ClientMessages::MovementInput {
        up: player_input.up,
        down: player_input.down,
        left: player_input.left,
        right: player_input.right,
    }) {
        client.send_message(DefaultChannel::Unreliable, input_message);
    }
}

pub(crate) fn change_position(
    mut change_pos_ev: EventReader<ChangePositions>,
    lobby: Res<Lobby>,
    mut commands: Commands,
) {
    for event in change_pos_ev.read() {
        let players = &event.0;
        for (player_id, transition) in players.iter() {
            let Some(player_entity) = lobby.players.get(player_id) else {
                continue;
            };
            let [x, y] = *transition;
            let transform = Transform {
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            };
            commands.entity(*player_entity).insert(transform);
        }
    }
}
