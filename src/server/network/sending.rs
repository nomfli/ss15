use crate::{
    make_log,
    server::logic::movement::Positions,
    shared::{
        components::{Grabbable, Player, Speed},
        messages::ServerMessages,
        sprites::SpriteName,
        utils::Loggable,
    },
};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub(crate) struct ServerSendPlug;

impl Plugin for ServerSendPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<SendServerMessage>();
        app.add_systems(Update, init_items);
        app.add_systems(Update, send_speed);
        app.add_systems(Update, send_msg::<SendServerMessage>);
        app.add_systems(Update, server_sync_players_movement);
        app.add_event::<SendItems>();
    }
}

#[derive(Event)]
pub(crate) struct SendItems {
    pub client_id: u64,
}

pub(crate) trait ServerMessage: Send + Sync + 'static {
    fn channel(&self) -> u8;
    fn get_recipient(&self) -> MessageRecipient;
    fn make_msg(&self) -> ServerMessages;
}

#[derive(Debug, Clone)]
pub(crate) enum MessageRecipient {
    Client(ClientId),
    Broadcast,
    #[allow(dead_code)]
    Group(Vec<ClientId>),
}

#[derive(Event, Debug, Clone)]
pub(crate) struct SendServerMessage {
    pub channel: u8,
    pub recipient: MessageRecipient,
    pub msg: ServerMessages,
}

impl ServerMessage for SendServerMessage {
    fn channel(&self) -> u8 {
        self.channel
    }
    fn make_msg(&self) -> ServerMessages {
        self.msg.clone()
    }
    fn get_recipient(&self) -> MessageRecipient {
        self.recipient.clone()
    }
}

pub(crate) fn send_msg<T: ServerMessage + Event>(
    mut reader: EventReader<T>,
    mut server: ResMut<RenetServer>,
) {
    reader.read().for_each(|event| {
        make_log!(bincode::serialize(&event.make_msg()), "serialize event msg").map(|msg| {
            match event.get_recipient() {
                MessageRecipient::Client(val) => {
                    server.send_message(val, event.channel(), msg);
                }
                MessageRecipient::Broadcast => {
                    server.broadcast_message(event.channel(), msg);
                }
                MessageRecipient::Group(users) => {
                    users
                        .iter()
                        .for_each(|user| server.send_message(*user, event.channel(), msg.clone()));
                }
            }
        });
    });
}

pub(crate) fn init_items(
    //init items for just connected player
    mut send_item: EventReader<SendItems>,
    items: Query<(&Transform, &SpriteName, Entity, &Grabbable), Without<Player>>,
    mut writer: EventWriter<SendServerMessage>,
) {
    for event in send_item.read() {
        for item in items.iter() {
            let (trans, name, ent, grabbable) = item;
            let Vec2 { x, y } = trans.translation.truncate();
            let item_msg = &ServerMessages::AddItem(([x, y], name.clone(), ent, *grabbable));

            writer.write(SendServerMessage {
                channel: DefaultChannel::Unreliable.into(),
                recipient: MessageRecipient::Client(event.client_id),
                msg: item_msg.clone(),
            });
        }
    }
}

pub(crate) fn send_speed(
    query: Query<(&Player, &Speed)>,
    mut writer: EventWriter<SendServerMessage>,
) {
    query.iter().for_each(|(player, speed)| {
        writer.write(SendServerMessage {
            channel: DefaultChannel::Unreliable.into(),
            recipient: MessageRecipient::Client(player.id),
            msg: ServerMessages::Speed(*speed),
        });
    });
}

pub(crate) fn server_sync_players_movement(
    query: Query<(&Transform, &Player)>,
    mut players: ResMut<Positions>,
    mut writer: EventWriter<SendServerMessage>,
) {
    query.iter().for_each(|(transform, player)| {
        players
            .0
            .insert(player.id, transform.translation.truncate().into());
    });
    writer.write(SendServerMessage {
        channel: DefaultChannel::Unreliable.into(),
        recipient: MessageRecipient::Broadcast,
        msg: ServerMessages::SendPositions(players.0.clone()),
    });
}
