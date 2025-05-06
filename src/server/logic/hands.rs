use crate::shared::{
    components::{Grabbable, Hands},
    events::ThrowAnswerEvent,
};

use bevy::prelude::*;
use bevy_renet::renet::*;

pub struct HandsServerPlug;

impl Plugin for HandsServerPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<GrabEvent>();
        app.add_event::<GrabAnsEvent>();
        app.add_event::<ThrowEvent>();
        app.add_systems(Update, grab_answer_handler);
        app.add_systems(Update, throw_answer);
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
pub(crate) struct GrabAnsEvent {
    pub can_be_grabbed: Entity,
    pub client: ClientId,
}

pub fn grab_answer_handler(
    mut grab_ev: EventReader<GrabEvent>,
    mut i_want_grab: Query<(&Transform, &mut Hands)>,
    can_be_grabbed: Query<(&Transform, &Grabbable)>,
    mut send_grab_ev: EventWriter<GrabAnsEvent>,
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
                        send_grab_ev.write(GrabAnsEvent {
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

#[derive(Event, Debug)]
pub(crate) struct ThrowEvent {
    pub client: ClientId,
    pub selected_idx: usize,
    pub i_want_throw: Entity,
    pub where_throw: Vec2,
}

pub(crate) fn throw_answer(
    mut throw_ev: EventReader<ThrowEvent>,
    mut answer: EventWriter<ThrowAnswerEvent>,
    mut i_want_throw: Query<(&Transform, &mut Hands)>,
    mut commands: Commands,
) {
    for event in throw_ev.read() {
        if let Ok((trans, mut hands)) = i_want_throw.get_mut(event.i_want_throw) {
            let Some(grab_ent) = hands.all_hands[event.selected_idx].grab_ent else {
                continue;
            };
            let distance = event.where_throw - trans.translation.truncate();
            let res_throw_pos = if distance.length() < hands.all_hands[event.selected_idx].hand_len
            {
                event.where_throw
            } else {
                distance.normalize() * hands.all_hands[event.selected_idx].hand_len
                    + trans.translation.truncate()
            };
            let Vec2 { x, y } = res_throw_pos;
            hands.all_hands[event.selected_idx].grab_ent = None;
            commands.entity(grab_ent).insert(Transform {
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            });
            answer.send(ThrowAnswerEvent {
                hand_idx: event.selected_idx,
                client: event.client,
                where_throw: [x, y],
            });
        }
    }
}
