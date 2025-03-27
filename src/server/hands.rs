use crate::shared::{
    components::{Grabbable, Hands},
    messages::ServerMessages,
};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub struct HandsServerPlug;

impl Plugin for HandsServerPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<GrabEvent>();
        app.add_event::<ServerThrowEvent>();
        app.add_systems(Update, send_grabb_answer);
        app.add_systems(Update, send_throw_answer);
    }
}

#[derive(Event, Debug)]
pub struct GrabEvent {
    pub i_want_grabb: Entity,
    pub can_be_grabbed: Entity,
    pub hand_idx: usize,
    pub client: ClientId,
}

#[derive(Event, Debug)]
pub struct ServerThrowEvent {
    pub i_want_throw: Entity,
    pub hand_idx: usize,
    pub client: ClientId,
    pub where_throw: Vec2,
}

pub fn send_grabb_answer(
    mut server: ResMut<RenetServer>,
    mut grab_ev: EventReader<GrabEvent>,
    mut i_want_grabb: Query<(&Transform, &mut Hands)>,
    can_be_grabbed: Query<(&Transform, &Grabbable)>,
) {
    for event in grab_ev.read() {
        if let Ok((trans, mut hands)) = i_want_grabb.get_mut(event.i_want_grabb) {
            if let Ok((pos, grabbable)) = can_be_grabbed.get(event.can_be_grabbed) {
                if !grabbable.0 {
                    continue;
                }
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

pub fn send_throw_answer(
    mut server: ResMut<RenetServer>,
    mut throw_ev: EventReader<ServerThrowEvent>,
    mut i_want_throw: Query<(&Transform, &mut Hands)>,
    mut commands: Commands,
) {
    for event in throw_ev.read() {
        if let Ok((trans, mut hands)) = i_want_throw.get_mut(event.i_want_throw) {
            let Some(grabb_ent) = hands.all_hands[event.hand_idx].grabb_ent else {
                continue;
            };
            let distance = event.where_throw - trans.translation.truncate();
            let res_throw_pos = if distance.length() < hands.all_hands[event.hand_idx].hand_len {
                event.where_throw
            } else {
                distance.normalize() * hands.all_hands[event.hand_idx].hand_len
                    + trans.translation.truncate()
            };
            let Vec2 { x, y } = res_throw_pos;
            hands.all_hands[event.hand_idx].grabb_ent = None;
            commands.entity(grabb_ent).insert(Transform {
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            });
            let Ok(throw_msg) = bincode::serialize(&ServerMessages::ThrowAnswer(
                grabb_ent,
                event.client,
                [x, y],
            )) else {
                continue;
            };
            server.broadcast_message(DefaultChannel::Unreliable, throw_msg);
        }
    }
}
