use crate::shared::{
    components::{PlayerEntity, Speed},
    resource::Lobby,
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


#[derive(Default, Debug, Clone, Resource)]
pub(crate) struct ChangePositions(pub HashMap<Entity, [f32; 2]>);

pub(crate) fn change_position(change_pos: Res<ChangePositions>, mut commands: Commands) {
    for (ent, pos) in change_pos.0.iter() {
        let [x, y] = *pos;

        let transform = Transform {
            translation: Vec3::new(x, y, 0.0),
            ..Default::default()
        };
        commands.entity(*ent).insert(transform);

    }
}


#[derive(Default, Debug, Clone, Event)]
pub(crate) struct SpeedEvent(pub Speed);

pub(crate) fn change_speed(
    mut speed_ev: EventReader<SpeedEvent>,
    query: Query<(Entity, &PlayerEntity)>,
    mut commands: Commands,
) {
    for event in speed_ev.read() {
        for (ent, _) in query.iter() {
            commands.entity(ent).insert(event.0);
        }
    }
}

