use crate::shared::{
    components::Speed,
    map::Collisionable,
    sprites::{SpriteName, Sprites},
};
use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

pub(crate) struct ServerCollisionPlug;

impl Plugin for ServerCollisionPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_collisions);
    }
}

pub(crate) enum Collision {
    Up,
    Bottom,
    Left,
    Right,
}

pub(crate) fn check_collisions(
    collision_obj: Query<(&Transform, &SpriteName), With<Collisionable>>,
    mut moving_entities: Query<(&mut Transform, &SpriteName, &mut Speed), Without<Collisionable>>,
    sprites: Res<Sprites>,
    time: Res<Time>,
) {
    let colliders: Vec<Aabb2d> = collision_obj
        .iter()
        .filter_map(|(transform, name)| {
            sprites.0.get(&name.0).and_then(|sprite| {
                sprite
                    .custom_size
                    .or_else(|| sprite.rect.map(|r| r.size()))
                    .map(|size| {
                        let pos = transform.translation.truncate();
                        Aabb2d {
                            min: pos - size / 2.0,
                            max: pos + size / 2.0,
                        }
                    })
            })
        })
        .collect();

    for (mut transform, name, mut speed) in moving_entities.iter_mut() {
        let Some(sprite) = sprites.0.get(&name.0) else {
            continue;
        };
        let Some(item_size) = sprite.custom_size.or_else(|| sprite.rect.map(|r| r.size())) else {
            continue;
        };

        let item_half_size = item_size / 2.0;
        let mut position = transform.translation.truncate();
        let velocity = Vec2::new(speed.x, speed.y) * time.delta_secs();

        position.x += velocity.x;
        let aabb_x = Aabb2d {
            min: position - item_half_size,
            max: position + item_half_size,
        };

        let mut x_collision = false;
        for collider in &colliders {
            if aabb_x.intersects(collider) {
                x_collision = true;
                break;
            }
        }

        if x_collision {
            position.x -= 2.0 * velocity.x;
            speed.x = 0.0;
        }

        position.y += velocity.y;
        let aabb_y = Aabb2d {
            min: position - item_half_size,
            max: position + item_half_size,
        };

        let mut y_collision = false;
        for collider in &colliders {
            if aabb_y.intersects(collider) {
                y_collision = true;
                break;
            }
        }

        if y_collision {
            position.y -= 2.0 * velocity.y;
            speed.y = 0.0;
        }

        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}
