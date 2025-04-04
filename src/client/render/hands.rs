use crate::{
    client::render::input::Mouse,
    shared::{
        components::{Grabbable, Hands, PlayerEntity},
        resource::{Entities, Lobby},
    },
};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub struct HandsClientPlug;

impl Plugin for HandsClientPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<ShouldGrabb>();
        app.add_event::<TryToGrabbEvent>();
        app.add_systems(Update, change_hand);
        app.add_systems(Update, try_to_grabb);
        app.add_systems(Update, grab_event_handler);
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
pub struct TryToGrabbEvent {
    pub can_be_grabbed: Entity, //server Entity
    pub hand_idx: usize,
}

pub fn try_to_grabb(
    i_want_grabb: Query<(&Hands, &PlayerEntity)>,
    can_be_grabbed: Query<(&Transform, &Sprite, Entity, &Grabbable)>,
    entities: Res<Entities>,
    mouse_input: Res<Mouse>,
    mut writer: EventWriter<TryToGrabbEvent>,
) {
    for (hands, _) in i_want_grabb.iter() {
        let selected_idx = hands.selected_hand;
        if hands.all_hands[selected_idx].grabb_ent.is_some() {
            return;
        }
        let Some(cur_pos) = mouse_input.cords else {
            return;
        };
        for (coords, sprite, ent, grabbable) in can_be_grabbed.iter() {
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
                writer.send(TryToGrabbEvent {
                    can_be_grabbed: *server_ent,
                    hand_idx: selected_idx,
                });
            }
        }
    }
}
#[derive(Event, Debug)]
pub struct ShouldGrabb {
    pub i_must_be_grabbed: Entity,
    pub who_should_grabe: ClientId,
}
pub fn grab_event_handler(
    lobby: Res<Lobby>,
    entities: Res<Entities>,
    mut grab_event: EventReader<ShouldGrabb>,
    mut query: Query<&mut Hands>,
    mut commands: Commands,
) {
    for event in grab_event.read() {
        let Some(&player_entity) = lobby.players.get(&event.who_should_grabe) else {
            continue;
        };
        let Ok(mut hands) = query.get_mut(player_entity) else {
            continue;
        };
        let Some(&must_be_grabbed) = entities.entities.get_by_second(&event.i_must_be_grabbed)
        else {
            continue;
        };
        if commands.get_entity(must_be_grabbed).is_none() {
            continue;
        }

        let selected_idx = hands.selected_hand;
        hands.all_hands[selected_idx].grabb_ent = Some(must_be_grabbed);

        commands
            .entity(must_be_grabbed)
            .remove::<Sprite>()
            .remove::<Transform>();
    }
}
