use crate::shared::components::{Grabbable, Hands};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub struct HandsServerPlug;

impl Plugin for HandsServerPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<GrabEvent>();
        app.add_event::<GrabAnsEv>();
        app.add_systems(Update, grab_answer_handler);
    }
}

#[derive(Event, Debug)]
pub struct GrabEvent {
    pub i_want_grab: Entity,
    pub can_be_grabbed: Entity,
    pub hand_idx: usize,
    pub client: ClientId,
}

#[derive(Event, Debug)]
pub(crate) struct GrabAnsEv {
    pub can_be_grabbed: Entity,
    pub client: ClientId,
}

pub fn grab_answer_handler(
    mut grab_ev: EventReader<GrabEvent>,
    mut i_want_grab: Query<(&Transform, &mut Hands)>,
    can_be_grabbed: Query<(&Transform, &Grabbable)>,
    mut send_grab_ev: EventWriter<GrabAnsEv>,
    mut commands: Commands,
) {
    for event in grab_ev.read() {
        if let Ok((trans, mut hands)) = i_want_grab.get_mut(event.i_want_grab) {
            if let Ok((pos, grabbable)) = can_be_grabbed.get(event.can_be_grabbed) {
                if !grabbable.0 {
                    continue;
                }
                if hands.all_hands[event.hand_idx].grab_ent.is_none()
                    && (trans.translation.truncate() - pos.translation.truncate()).length()
                        < hands.all_hands[event.hand_idx].hand_len
                {
                    {
                        send_grab_ev.send(GrabAnsEv {
                            can_be_grabbed: event.can_be_grabbed,
                            client: event.client,
                        });

                        hands.all_hands[event.hand_idx].grab_ent = Some(event.can_be_grabbed);
                        commands
                            .entity(event.can_be_grabbed)
                            .remove::<Transform>()
                            .remove::<Sprite>();
                    };
                }
            }
        }
    }
}
