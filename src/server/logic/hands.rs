use crate::{
    server::logic::movement::Speed,
    shared::{
        components::{Grabbable, Hands},
        events::ThrowAnswerEvent,
        resource::Lobby,
    },
};

use bevy::prelude::*;
use bevy_renet::renet::*;

pub struct HandsServerPlug;

impl Plugin for HandsServerPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<GrabEvent>();
        app.add_event::<GrabAnsEvent>();
        app.add_event::<ThrowEvent>();
        app.add_event::<ThrowAwayEvent>();
        app.add_event::<ThrowAwayAnswerEvent>();
        app.add_systems(Update, grab_answer_handler);
        app.add_systems(Update, throw_answer);
        app.add_systems(Update, throw_away_handled);
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
            answer.write(ThrowAnswerEvent {
                hand_idx: event.selected_idx,
                client: event.client,
                where_throw: [x, y],
            });
        }
    }
}

#[derive(Event, Debug)]
pub(crate) struct ThrowAwayEvent {
    pub where_throw: Vec2,
    pub hand_idx: usize,
    pub client_id: ClientId,
}

pub(crate) fn throw_away_handled(
    mut throw_away_ev: EventReader<ThrowAwayEvent>,
    mut i_want_throw_away: Query<(&mut Hands, &Transform)>,
    mut i_want_freedom: Query<&mut Speed>,
    lobby: Res<Lobby>,
    mut commands: Commands,
    mut writer: EventWriter<ThrowAwayAnswerEvent>,
) {
    for event in throw_away_ev.read() {
        let Some(player_ent) = lobby.players.get(&event.client_id) else {
            continue;
        };
        let Ok((mut hands, player_transform)) = i_want_throw_away.get_mut(*player_ent) else {
            continue;
        };
        let Some(grabbed_ent) = hands.all_hands[event.hand_idx].grab_ent else {
            continue;
        };
        hands.all_hands[event.hand_idx].grab_ent = None;
        let Ok(mut speed) = i_want_freedom.get_mut(grabbed_ent) else {
            continue;
        };
        commands
            .entity(grabbed_ent)
            .insert(player_transform.clone());
        let direction = event.where_throw - player_transform.translation.truncate();
        let item_speed =
            Vec2::new(direction.x.sqrt(), direction.y.sqrt()) * ((2.0 * 0.95) as f32).sqrt();
        speed.x = item_speed.x;
        speed.y = item_speed.y;
        writer.write(ThrowAwayAnswerEvent {
            client_id: event.client_id,
            hand_idx: event.hand_idx,
        });
    }
}

#[derive(Event, Debug)]
pub(crate) struct ThrowAwayAnswerEvent {
    pub client_id: ClientId,
    pub hand_idx: usize,
}
