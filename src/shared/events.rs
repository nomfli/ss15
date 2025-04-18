use bevy::prelude::*;
use bevy_renet::renet::ClientId;

pub(crate) struct SharedEvents;

impl Plugin for SharedEvents {
    fn build(&self, app: &mut App) {
        app.add_event::<ThrowAnswerEvent>();
    }
}

#[derive(Event, Debug)]
pub(crate) struct ThrowAnswerEvent {
    pub hand_idx: usize,
    pub client: ClientId,
    pub where_throw: [f32; 2],
}
