use crate::server::logic::movement::Positions;
use bevy::prelude::*;

pub(crate) struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

trait Measure {
    fn distance(first_coords: Vec2, second_coord: Vec2) -> f32 {
        (first_coords - second_coord).length()
    }

    fn entities_in_radius(&self, circle: &Circle, movement: Res<Positions>) -> Vec<Entity> {
        movement
            .0
            .iter()
            .filter_map(|(ent, [x, y])| {
                if Self::distance(circle.center, Vec2::new(*x, *y)) < circle.radius {
                    Some(ent.clone()) // Предполагаем, что Entity реализует Clone
                } else {
                    None
                }
            })
            .collect()
    }
}
