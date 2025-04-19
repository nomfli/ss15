use crate::server::logic::movement::Positions;
use bevy::prelude::*;

pub(crate) struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

pub(crate) trait Measure {
    fn distance(first_coords: Vec2, second_coord: Vec2) -> f32 {
        (first_coords - second_coord).length()
    }

    fn entities_in_radius(&self, circle: &Circle, positions: &Positions) -> Vec<Entity> {
        positions
            .0
            .iter()
            .filter_map(|(entity, &[x, y])| {
                if Self::distance(circle.center, Vec2::new(x, y)) < circle.radius {
                    Some(Entity::from_bits(*entity))
                } else {
                    None
                }
            })
            .collect()
    }
}
