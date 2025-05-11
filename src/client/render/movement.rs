use crate::shared::{
    components::{PlayerEntity, Speed},
    resource::Entities,
};

use bevy::prelude::*;
use std::collections::HashMap;

pub struct MovementClientPlug;

impl Plugin for MovementClientPlug {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangePositions>();
        app.add_event::<SpeedEvent>();
        app.add_systems(Update, change_position);
        app.add_systems(Update, change_speed);
    }
}

#[derive(Default, Debug, Clone, Event)]
pub(crate) struct ChangePositions(pub HashMap<Entity, [f32; 2]>);

pub(crate) fn change_position(
    mut change_pos_ev: EventReader<ChangePositions>,
    entities: Res<Entities>,
    mut commands: Commands,
) {
    for event in change_pos_ev.read() {
        let players = &event.0;
        for (player_id, transition) in players.iter() {
            let Some(player_entity) = entities.entities.get_by_second(player_id) else {
                continue;
            };
            let [x, y] = *transition;
            let transform = &Transform {
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            };
            commands.entity(*player_entity).insert(*transform);
        }
    }
}

#[derive(Default, Debug, Clone, Event)]
pub(crate) struct SpeedEvent(pub Speed);

pub(crate) fn change_speed(
    mut speed_ev: EventReader<SpeedEvent>,
    query: Query<Entity, With<PlayerEntity>>,
    mut commands: Commands,
) {
    for event in speed_ev.read() {
        for ent in query.iter() {
            commands.entity(ent).insert(event.0);
        }
    }
}
