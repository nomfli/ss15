use crate::{
    make_log,
    shared::{
        components::{Direction, PlayerEntity},
        messages::ClientMessages,
        resource::MovementInput,
        utils::Loggable,
    },
};
use std::{fmt::Debug, marker::Sync};

use bevy::prelude::*;
use bevy_renet::renet::*;

pub(crate) struct ClientSendingPlug;

impl Plugin for ClientSendingPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<SendMessage<u8>>();
        app.add_systems(Update, send_resource::<MovementInput>);
        app.add_systems(Update, send_query::<Direction, PlayerEntity>);
        app.add_systems(Update, sending_event_to_server::<u8>);
    }
}

pub(crate) trait MakeMessage {
    fn make_msg(&self) -> ClientMessages;
    fn channel(&self) -> u8;
}

#[derive(Event, Debug)]
pub(crate) struct SendMessage<T: Into<u8> + Debug + Sync + Send + 'static + Copy> {
    pub msg: ClientMessages,
    pub channel: T,
}

pub(crate) fn sending_event_to_server<T: Into<u8> + Debug + Sync + Send + 'static + Copy>(
    mut msg_ev: EventReader<SendMessage<T>>,
    mut client: ResMut<RenetClient>,
) {
    msg_ev
        .read()
        .filter_map(|x| {
            make_log!(bincode::serialize(&x.msg), "serilize event msg").map(|y| (x.channel, y))
        })
        .for_each(|(channel, msg)| client.send_message(channel.into(), msg));
}

pub fn send_resource<T: Resource + MakeMessage>(resource: Res<T>, mut client: ResMut<RenetClient>) {
    make_log!(
        bincode::serialize(&resource.make_msg()),
        "serilize resource msg"
    )
    .map(|x| client.send_message(resource.channel(), x));
}

pub fn send_query<T: MakeMessage + Component, U: Component>(
    query: Query<&T, With<U>>,
    mut client: ResMut<RenetClient>,
) {
    query
        .iter()
        .filter_map(|x| {
            make_log!(bincode::serialize(&x.make_msg()), "serialize query")
                .map(|y| (x.channel(), y))
        })
        .for_each(|(ch, msg)| client.send_message(ch, msg));
}

impl MakeMessage for MovementInput {
    fn make_msg(&self) -> ClientMessages {
        ClientMessages::MovementInput {
            up: self.up,
            down: self.down,
            left: self.left,
            right: self.right,
        }
    }
    fn channel(&self) -> u8 {
        DefaultChannel::Unreliable.into()
    }
}

impl MakeMessage for Direction {
    fn channel(&self) -> u8 {
        DefaultChannel::Unreliable.into()
    }
    fn make_msg(&self) -> ClientMessages {
        ClientMessages::Direction(*self)
    }
}
