use crate::client::input::Mouse;
use crate::shared::{
    components::{Hands, PlayerEntity, ServerEntityId},
    messages::ClientMessages,
    resource::Lobby,
};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub struct HandsClientPlug;

impl Plugin for HandsClientPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<ShouldGrabb>();
        app.add_systems(Update, change_hand);
        app.add_systems(Update, try_to_grabb);
        app.add_systems(Update, grab_event_handler);
    }
}

#[derive(Event)]
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
    can_be_grabbed: Query<(&Transform, &Sprite, &ServerEntityId)>,
    mouse_input: Res<Mouse>,
    mut client: ResMut<RenetClient>,
) {
    for (hand, _) in i_want_grabb.iter() {
        let selected_idx = hand.selected_hand;

        for (coords, sprite, ent) in can_be_grabbed.iter() {
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
                        hands.all_hands[selected_idx].grabb_ent = Some(i_must_be_grabbed);
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
