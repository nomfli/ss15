use crate::{
    server::{
        logic::movement::Positions,
        physics::measure::{Circle, Measure},
    },
    shared::chat::ChatMode,
};
use bevy::prelude::*;

pub const SAY_RADIUS: f32 = 5000.0;

#[derive(Event, Debug, Clone)]
pub(crate) struct MsgHandlerEvent {
    pub mode: ChatMode,
    pub text: String,
    pub client_ent: Entity,
}

impl Measure for MsgHandlerEvent {}

pub(crate) fn handle_chat_msg(
    mut handle_msg_ev: EventReader<MsgHandlerEvent>,
    query: Query<&Transform>,
    positions: Res<Positions>,
) {
    for event in handle_msg_ev.read() {
        let transform = query.get(event.client_ent);
        match event.mode {
            ChatMode::Say => {
                let Ok(transform) = query.get(event.client_ent) else {
                    continue;
                };
                let center = transform.translation.truncate();
                let radius = SAY_RADIUS;
                (*event).entities_in_radius(&Circle { center, radius }, &positions);
            }
            _ => {} //TODO
        }
    }
}
