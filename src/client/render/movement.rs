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
        app.add_systems(Update, interpolation_system);
        app.add_systems(Update, change_speed);
    }
}

#[derive(Component, Debug, Clone)]
pub(crate) struct InterpolationTarget {
    pub start: Vec2,
    pub end: Vec2,
    pub progress: f32,
    pub duration: f32,
}

#[derive(Default, Debug, Clone, Event)]
pub(crate) struct ChangePositions(pub HashMap<u64, [f32; 2]>);

pub(crate) fn change_position(
    mut change_pos_ev: EventReader<ChangePositions>,
    lobby: Res<Lobby>,
    mut interpolation: Query<&mut InterpolationTarget>,
    mut commands: Commands,
    mut transforms: Query<&mut Transform>,
) {
    for event in change_pos_ev.read() {
        let players = &event.0;
        for (player_id, position) in players.iter() {
            let Some(player_entity) = lobby.players.get(player_id) else {
                continue;
            };

            let [x, y] = *position;
            let new_pos = Vec2::new(x, y);

            if let Ok(mut transform) = transforms.get_mut(*player_entity) {
                if let Ok(mut interpolation) = interpolation.get_mut(*player_entity) {
                    interpolation.start =
                        Vec2::new(transform.translation.x, transform.translation.y);
                    interpolation.end = new_pos;
                    interpolation.progress = 0.0;
                } else {
                    commands.entity(*player_entity).insert(InterpolationTarget {
                        start: Vec2::new(transform.translation.x, transform.translation.y),
                        end: new_pos,
                        progress: 1.0,
                        duration: 0.1,
                    });
                    transform.translation =
                        Vec3::new(new_pos.x, new_pos.y, transform.translation.z);
                }
            } else {
                commands
                    .entity(*player_entity)
                    .insert(InterpolationTarget {
                        start: new_pos,
                        end: new_pos,
                        progress: 1.0,
                        duration: 0.1,
                    })
                    .insert(Transform {
                        translation: Vec3::new(new_pos.x, new_pos.y, 0.0),
                        ..Default::default()
                    });
            }
        }
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

pub(crate) fn interpolation_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut InterpolationTarget)>,
) {
    for (mut transform, mut interpolation) in query.iter_mut() {
        if interpolation.progress < 1.0 {
            interpolation.progress += time.delta_secs() / interpolation.duration;
            interpolation.progress = interpolation.progress.min(1.0);

            let new_pos = interpolation
                .start
                .lerp(interpolation.end, interpolation.progress);
            transform.translation = Vec3::new(new_pos.x, new_pos.y, transform.translation.z);
        }
    }
}
