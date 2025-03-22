use crate::shared::hands::*;
use crate::shared::*;
use bevy::prelude::*;

pub const GRAB_RADIUS: f32 = 100.0;

pub fn change_hands(mut query: Query<(&PlayerInput, &mut HandsCharacter)>) {
    for (input, mut hand) in query.iter_mut() {
        if input.change_hand {
            hand.selected = (hand.selected + 1) % hand.hands.len();
        }
    }
}

pub fn grabb(
    i_want_grabb: Query<(&PlayerInput, &HandsCharacter, &Transform, Entity)>,
    mut i_wanna_be_grabed: Query<(&mut IAmGrabbed, &Transform, &SpriteName, Entity)>,
    data: Res<Data>,
    mut commands: Commands,
) {
    for (input, hand, trans, it_me) in i_want_grabb.iter() {
        let mut selected_hand: Box<Hand> = Box::new(hand.hands[hand.selected]);
        if input.left_mouse && selected_hand.grabbed_entity.is_none() {
            for (mut i_am_grabbed, coords, name, ent) in i_wanna_be_grabed.iter_mut() {
                if it_me != ent {
                    if let Some(sprite) = data.sprite.get(&name.0) {
                        let half_size = sprite.custom_size.unwrap_or(Vec2::new(128.0, 128.0)) * 0.5;
                        let sprite_position = coords.translation.truncate();
                        if let Some(cur_pos) = input.cursor_pos {
                            if cur_pos.x >= sprite_position.x - half_size.x
                                && cur_pos.x <= sprite_position.x + half_size.x
                                && cur_pos.y >= sprite_position.y - half_size.y
                                && cur_pos.y <= sprite_position.y + half_size.y
                            {
                                if (cur_pos - trans.translation.truncate()).length() < GRAB_RADIUS {
                                    i_am_grabbed.0 = true;
                                    selected_hand.grabbed_entity = Some(ent);
                                    commands.entity(ent).remove::<Transform>();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn throw(
    mut i_want_throw: Query<(&PlayerInput, &mut HandsCharacter, &Transform)>,
    mut i_am_grabbed_query: Query<(&mut IAmGrabbed, &mut Transform)>,
) {
    for (input, hand, transform) in i_want_throw.iter_mut() {
        let mut selected_hand: Box<Hand> = Box::new(hand.hands[hand.selected]);
        if input.throw && selected_hand.grabbed_entity.is_some() {
            if let Some(cur_pos) = input.cursor_pos {
                if let Some(grabbed_ent) = selected_hand.grabbed_entity {
                    if let Ok((mut i_am_grabbed, mut position)) =
                        i_am_grabbed_query.get_mut(grabbed_ent)
                    {
                        let distance = cur_pos - transform.translation.truncate();

                        if distance.length() < GRAB_RADIUS {
                            i_am_grabbed.0 = false;
                            position.translation = Vec3::new(cur_pos.x, cur_pos.y, 0.0);
                            (*selected_hand).grabbed_entity = None;
                        } else {
                            let throw_dir = distance.normalize();
                            let throw_pos =
                                transform.translation.truncate() + throw_dir * GRAB_RADIUS;
                            i_am_grabbed.0 = false;
                            position.translation = Vec3::new(throw_pos.x, throw_pos.y, 0.0);
                        }
                    }
                }
            }
        }
    }
}
