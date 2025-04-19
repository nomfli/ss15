use bevy::prelude::*;
use std::collections::HashMap;

pub struct MovementClientPlug;

impl Plugin for MovementClientPlug {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChangePositions>();
        app.add_systems(Update, change_position);
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
