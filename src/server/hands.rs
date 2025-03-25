use crate::shared::{components::Hands, messages::ServerMessages};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub struct HandsServerPlug;

impl Plugin for HandsServerPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<GrabEvent>();
        app.add_systems(Update, send_grabb_answer);
    }
}

#[derive(Event, Debug)]
pub struct GrabEvent {
    pub i_want_grabb: Entity,
    pub can_be_grabbed: Entity,
    pub hand_idx: usize,
    pub client: ClientId,
}

pub fn send_grabb_answer(
    mut server: ResMut<RenetServer>,
    mut grab_ev: EventReader<GrabEvent>,
    mut i_want_grabb: Query<(&Transform, &mut Hands)>,
    can_be_grabbed: Query<&Transform>,
) {
    for event in grab_ev.read() {
        if let Ok((trans, mut hands)) = i_want_grabb.get_mut(event.i_want_grabb) {
            if let Ok(pos) = can_be_grabbed.get(event.can_be_grabbed) {
                if hands.all_hands[event.hand_idx].grabb_ent.is_none() {
                    if (trans.translation.truncate() - pos.translation.truncate()).length()
                        < hands.all_hands[event.hand_idx].hand_len
                    {
                        let Ok(sync_message) = bincode::serialize(&ServerMessages::GrabAnswer(
                            event.can_be_grabbed,
                            event.client,
                        )) else {
                            break;
                        };
                        hands.all_hands[event.hand_idx].grabb_ent = Some(event.can_be_grabbed);
                        server.broadcast_message(DefaultChannel::Unreliable, sync_message);
                    }
                }
            }
        }
    }
}
