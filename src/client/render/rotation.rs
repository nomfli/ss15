use crate::client::render::input::Mouse;
use crate::shared::{
    components::{Direction, PlayerEntity, Speed},
    messages::ClientMessages,
    resource::{Entities, MovementInput},
};
use bevy::prelude::*;
use bevy_renet::renet::*;

pub(crate) struct RotClientPlug;

impl Plugin for RotClientPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_direction);
        app.add_systems(Update, send_direction);
        app.add_systems(Update, add_rotation);
        app.add_systems(Update, render_rotation);
        app.add_event::<RotationClientEvent>();
    }
}

pub(crate) fn change_direction(
    mut query: Query<(&mut Direction, &Speed, &Transform, &PlayerEntity)>,
    input: Res<MovementInput>,
    mouse: Res<Mouse>,
) {
    for (mut direction, speed, trans, _) in query.iter_mut() {
        if input.up {
            *direction = Direction::Up;
        } else if input.down {
            *direction = Direction::Down;
        } else if input.left {
            *direction = Direction::Left;
        } else if input.right {
            *direction = Direction::Right;
        }
        if speed.x * speed.x + speed.y * speed.y < 0.1 {
            let Some(cursor_pos) = mouse.cords else {
                return;
            };
            if !mouse.left_button {
                continue;
            }
            let dir = cursor_pos - trans.translation.truncate();
            if (dir.y > dir.x) && (dir.y > -dir.x) {
                *direction = Direction::Up;
            }
            if (dir.y < dir.x) && (dir.y > -dir.x) {
                *direction = Direction::Right;
            }
            if (dir.y < dir.x) && (dir.y < -dir.x) {
                *direction = Direction::Down;
            }
            if (dir.y > dir.x) && (dir.y < -dir.x) {
                *direction = Direction::Left;
            }
        }
    }
}

pub(crate) fn send_direction(
    mut client: ResMut<RenetClient>,
    query: Query<(&PlayerEntity, &Direction)>,
) {
    for (_, dir) in query.iter() {
        let Ok(direction_msg) = bincode::serialize(&ClientMessages::Direction(*dir)) else {
            return;
        };
        client.send_message(DefaultChannel::Unreliable, direction_msg)
    }
}
#[derive(Event)]
pub(crate) struct RotationClientEvent {
    pub dir: Direction,
    pub server_ent: Entity,
}

pub(crate) fn add_rotation(
    mut rot_ev: EventReader<RotationClientEvent>,
    entities: Res<Entities>,
    mut command: Commands,
) {
    for event in rot_ev.read() {
        let Some(ent) = entities.entities.get_by_second(&event.server_ent) else {
            continue;
        };
        command.entity(*ent).insert(event.dir);
    }
}

pub(crate) fn render_rotation(mut query: Query<(&Direction, &mut Sprite)>) {
    for (direction, mut sprite) in query.iter_mut() {
        if let Some(ref mut atlas) = sprite.texture_atlas {
            atlas.index = usize::from(*direction);
        }
    }
}
