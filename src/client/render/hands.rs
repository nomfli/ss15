use crate::{
    client::render::input::Mouse,
    shared::{
        components::{Grabbable, Hands, PlayerEntity},
        events::ThrowAnswerEvent,
        resource::{Entities, Lobby},
        sprites::{SpriteName, Sprites},
    },
};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub struct HandsClientPlug;

impl Plugin for HandsClientPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<ShouldGrab>();
        app.add_event::<TryToGrabEvent>();
        app.add_event::<SendTryThrow>();
        app.add_systems(Update, throw);
        app.add_systems(Update, change_hand);
        app.add_systems(Update, try_to_grab);
        app.add_systems(Update, grab_event_handler);
        app.add_systems(Update, try_throw);
    }
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

#[derive(Event)]
pub struct TryToGrabEvent {
    pub can_be_grabbed: Entity, //server Entity
    pub hand_idx: usize,
}


pub fn try_to_grab(
    i_want_grab: Query<(&Hands, &PlayerEntity)>,
    can_be_grabed: Query<(&Transform, &Sprite, Entity, &Grabbable)>,
    entities: Res<Entities>,
    mouse_input: Res<Mouse>,
    mut writer: EventWriter<TryToGrabEvent>,
) {
    for (hands, _) in i_want_grab.iter() {
        let selected_idx = hands.selected_hand;
        if hands.all_hands[selected_idx].grab_ent.is_some() {
            return;
        }
        let Some(cur_pos) = mouse_input.cords else {
            return;
        };
        for (coords, sprite, ent, grabbable) in can_be_grabed.iter() {
            if !grabbable.0 {
                continue;
            }
            let half_size = sprite.custom_size.unwrap_or(Vec2::new(128.0, 128.0)) * 0.5;
            let sprite_position = coords.translation.truncate();

            if cur_pos.x >= sprite_position.x - half_size.x
                && cur_pos.x <= sprite_position.x + half_size.x
                && cur_pos.y >= sprite_position.y - half_size.y
                && cur_pos.y <= sprite_position.y + half_size.y
                && mouse_input.left_button
            {
                let Some(server_ent) = entities.entities.get_by_first(&ent) else {
                    panic!("problem with bimap entities can't find entity");
                };

                writer.write(TryToGrabEvent {
                    can_be_grabbed: *server_ent,
                    hand_idx: selected_idx,
                });
            }
        }
    }
}

#[derive(Event, Debug)]

pub struct ShouldGrab {
    pub i_must_be_grabbed: Entity,
    pub who_should_grab: ClientId,
}
pub fn grab_event_handler(
    lobby: Res<Lobby>,
    entities: Res<Entities>,
    mut grab_event: EventReader<ShouldGrab>,
    mut query: Query<&mut Hands>,
    mut commands: Commands,
) {
    for event in grab_event.read() {
        let Some(&player_entity) = lobby.players.get(&event.who_should_grab) else {
            continue;
        };
        let Ok(mut hands) = query.get_mut(player_entity) else {
            continue;
        };
        let Some(&must_be_grabbed) = entities.entities.get_by_second(&event.i_must_be_grabbed)
        else {
            continue;
        };
        if commands.get_entity(must_be_grabbed).is_err() {
            continue;
        }
        let selected_idx = hands.selected_hand;
        hands.all_hands[selected_idx].grab_ent = Some(must_be_grabbed);
        commands
            .entity(must_be_grabbed)
            .remove::<Sprite>()
            .remove::<Transform>();
    }
}

#[derive(Event)]
pub(crate) struct SendTryThrow {
    pub hand_idx: usize,
    pub where_throw: Vec2,
}

pub(crate) fn try_throw(
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<(&PlayerEntity, &Hands)>,
    mut send_ev: EventWriter<SendTryThrow>,
    mouse_input: Res<Mouse>,
) {
    for (_, hands) in query.iter() {
        let hand_idx = hands.selected_hand;
        if hands.all_hands[hand_idx].grab_ent.is_none() {
            return;
        }
        let Some(where_throw) = mouse_input.cords else {
            return;
        };
        if keyboard.pressed(KeyCode::KeyQ) {
            send_ev.write(SendTryThrow {
                hand_idx,
                where_throw,
            });
        }
    }
}

pub(crate) fn throw(
    mut reader: EventReader<ThrowAnswerEvent>,
    mut commands: Commands,
    mut hands_query: Query<&mut Hands>,
    sprite_query: Query<&SpriteName>,
    lobby: Res<Lobby>,
    sprites: Res<Sprites>,
) {
    for event in reader.read() {
        let Some(ent) = lobby.players.get(&event.client) else {
            continue;
        };
        let Ok(mut hands) = hands_query.get_mut(*ent) else {
            continue;
        };
        let Some(i_want_freedom) = hands.all_hands[event.hand_idx].grab_ent else {
            continue;
        };
        let [x, y] = event.where_throw;

        if let Ok(sprite_name) = sprite_query.get(i_want_freedom) {
            if let Some(sprite) = sprites.0.get(&sprite_name.0) {
                commands
                    .entity(i_want_freedom)
                    .insert(Transform {
                        translation: Vec3::new(x, y, 0.0),
                        ..Default::default()
                    })
                    .insert(sprite.clone());
                hands.all_hands[event.hand_idx].grab_ent = None;
            }
        }
    }
}
