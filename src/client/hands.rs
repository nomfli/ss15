use crate::client::input::{Mouse, ThrowInput};
use crate::shared::{
    components::{Grabbable, Hands, PlayerEntity, ServerEntityId},
    messages::ClientMessages,
    resource::Lobby,
    sprites::{SpriteName, Sprites},
};
use bevy::prelude::*;
use bevy_renet::netcode::NetcodeClientTransport;
use bevy_renet::renet::*;

pub struct HandsClientPlug;

impl Plugin for HandsClientPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<ClientThrowEvent>();
        app.add_event::<ShouldGrabb>();
        app.add_systems(Update, change_hand);
        app.add_systems(Update, try_to_grabb);
        app.add_systems(Update, grab_event_handler);
        app.add_systems(Update, try_to_throw);
        app.add_systems(Update, throw_event_handler);
    }
}

#[derive(Event, Debug)]
pub struct ClientThrowEvent {
    pub i_want_freedom: Entity,
    pub client_id: ClientId,
    pub where_throw: [f32; 2],
}

#[derive(Event, Debug)]
pub struct ShouldGrabb {
    pub i_must_be_grabbed: Entity,
    pub who_should_grabe: ClientId,
}
pub fn change_hand(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hands_q: Query<(&mut Hands, &PlayerEntity)>,
) {
    for (mut hands, _) in hands_q.iter_mut() {
        if keyboard.pressed(KeyCode::KeyX) {
            hands.selected_hand = (hands.selected_hand + 1) % hands.all_hands.len();
        }
    }
}

pub fn try_to_grabb(
    i_want_grabb: Query<(&Hands, &PlayerEntity)>,
    can_be_grabbed: Query<(&Transform, &Sprite, &ServerEntityId, &Grabbable)>,
    mouse_input: Res<Mouse>,
    mut client: ResMut<RenetClient>,
) {
    for (hand, _) in i_want_grabb.iter() {
        let selected_idx = hand.selected_hand;
        if hand.all_hands[selected_idx].grabb_ent.is_some() {
            continue;
        }
        for (coords, sprite, ent, grabbable) in can_be_grabbed.iter() {
            let half_size = sprite.custom_size.unwrap_or(Vec2::new(128.0, 128.0)) * 0.5;
            let sprite_position = coords.translation.truncate();
            let Some(cur_pos) = mouse_input.cords else {
                continue;
            };
            if cur_pos.x >= sprite_position.x - half_size.x
                && cur_pos.x <= sprite_position.x + half_size.x
                && cur_pos.y >= sprite_position.y - half_size.y
                && cur_pos.y <= sprite_position.y + half_size.y
                && mouse_input.left_button
            {
                if let Ok(grab_msg) = bincode::serialize(&ClientMessages::Grab {
                    can_be_grabbed: ent.ent_id,
                    hand_idx: selected_idx,
                }) {
                    client.send_message(DefaultChannel::Unreliable, grab_msg);
                }
            }
        }
    }
}

pub fn grab_event_handler(
    lobby: Res<Lobby>,
    mut grab_event: EventReader<ShouldGrabb>,
    mut query: Query<(&PlayerEntity, &mut Hands, Entity)>,
    i_must_be_grabbed: Query<(Entity, &ServerEntityId)>,
    mut command: Commands,
) {
    for event in grab_event.read() {
        let Some(who_should_grabbe) = lobby.players.get(&event.who_should_grabe) else {
            continue;
        };
        for (_, mut hands, ent) in query.iter_mut() {
            for (i_must_be_grabbed, server_ent) in i_must_be_grabbed.iter() {
                if server_ent.ent_id == event.i_must_be_grabbed {
                    if ent == *who_should_grabbe {
                        let selected_idx = hands.selected_hand;
                        hands.all_hands[selected_idx].grabb_ent = Some(server_ent.ent_id);
                        command
                            .entity(i_must_be_grabbed)
                            .remove::<(Sprite, Transform)>();
                    } else {
                        command
                            .entity(i_must_be_grabbed)
                            .remove::<(Sprite, Transform)>();
                    }
                }
            }
        }
    }
}

pub fn try_to_throw(
    mouse_input: Res<Mouse>,
    mut client: ResMut<RenetClient>,
    query: Query<(&Hands, &PlayerEntity)>,
    throw_input: Res<ThrowInput>,
) {
    for (hands, _) in query.iter() {
        if !throw_input.0 {
            continue;
        }
        let selected_idx = hands.selected_hand;
        let Some(mouse_coords) = mouse_input.cords else {
            continue;
        };
        let Ok(drop_msg) = bincode::serialize(&ClientMessages::Drop {
            hand_idx: selected_idx,
            where_throw: mouse_coords,
        }) else {
            continue;
        };
        client.send_message(DefaultChannel::Unreliable, drop_msg);
    }
}

pub fn throw_event_handler(
    mut throw_ev: EventReader<ClientThrowEvent>,
    query: Query<(Entity, &ServerEntityId, &SpriteName)>,
    mut who_throws: Query<&mut Hands>,
    mut command: Commands,
    client_transport: Res<NetcodeClientTransport>,
    lobby: Res<Lobby>,
    sprites: Res<Sprites>,
) {
    for event in throw_ev.read() {
        let this_client_id = client_transport.client_id();
        if this_client_id == event.client_id {
            let Some(player_ent) = lobby.players.get(&event.client_id) else {
                continue;
            };
            let Ok(mut hands) = who_throws.get_mut(*player_ent) else {
                continue;
            };

            let Some(hand_index) = hands
                .all_hands
                .iter()
                .position(|hand| hand.grabb_ent == Some(event.i_want_freedom))
            else {
                continue;
            };

            hands.all_hands[hand_index].grabb_ent = None;
        }
        let Some((ent, name)) = query
            .iter()
            .find(|(_, server_ent, _)| server_ent.ent_id == event.i_want_freedom)
            .map(|(e, _, name)| (e, name))
        else {
            continue;
        };

        let Some(sprite) = sprites.0.get(&name.0) else {
            continue;
        };
        let [x, y] = event.where_throw;
        command
            .entity(ent)
            .insert(Transform {
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            })
            .insert(sprite.clone());
    }
}
