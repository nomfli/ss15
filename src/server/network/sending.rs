use crate::{

    server::logic::hands::GrabAnsEvent,
    shared::{
        components::{Grabbable, Player, Speed},
        events::ThrowAnswerEvent,
        messages::ServerMessages,
        sprites::SpriteName,
    },
};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub(crate) struct ServerSendPlug;

impl Plugin for ServerSendPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, send_items);
        app.add_systems(Update, send_grab_answer);
        app.add_systems(Update, send_speed);
        app.add_systems(Update, send_throw_answer);
        app.add_event::<SendItems>();
    }
}

#[derive(Event)]
pub(crate) struct SendItems {
    pub client_id: u64,
}

pub(crate) fn send_items(
    mut send_item: EventReader<SendItems>,
    items: Query<(&Transform, &SpriteName, Entity, &Grabbable), Without<Player>>,
    mut server: ResMut<RenetServer>,
) {
    for event in send_item.read() {
        for item in items.iter() {
            let (trans, name, ent, grabbable) = item;
            let Vec2 { x, y } = trans.translation.truncate();
            let item_msg = bincode::serialize(&ServerMessages::AddItem((
                [x, y],
                name.clone(),
                ent,
                *grabbable,
            )));

            if let Ok(msg) = item_msg {
                server.send_message(event.client_id, DefaultChannel::Unreliable, msg)
            }
        }
    }
}

pub(crate) fn send_grab_answer(
    mut server: ResMut<RenetServer>,
    mut grab_ansewer: EventReader<GrabAnsEvent>,
) {
    for event in grab_ansewer.read() {
        let Ok(sync_message) = bincode::serialize(&ServerMessages::GrabAnswer(
            event.can_be_grabbed,
            event.client,
        )) else {
            continue;
        };
        server.broadcast_message(DefaultChannel::Unreliable, sync_message);
    }
}

pub(crate) fn send_speed(query: Query<(&Player, &Speed)>, mut server: ResMut<RenetServer>) {
    for (player, speed) in query.iter() {
        if let Ok(speed_msg) = bincode::serialize(&ServerMessages::Speed(*speed)) {
            server.send_message(player.id, DefaultChannel::Unreliable, speed_msg);
        }
    }
}


pub(crate) fn send_throw_answer(
    mut server: ResMut<RenetServer>,
    mut throw_answer: EventReader<ThrowAnswerEvent>,
) {
    for event in throw_answer.read() {
        let Ok(throw_msg) = bincode::serialize(&ServerMessages::ThrowAnswer {
            client_id: event.client,
            where_throw: event.where_throw,
            hand_idx: event.hand_idx,
        }) else {
            continue;
        };
        server.broadcast_message(DefaultChannel::Unreliable, throw_msg);
    }
}


