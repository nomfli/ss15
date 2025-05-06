use crate::shared::{
    map::{init_map_tmp, Map},
    sprites::{SpriteName, Sprites},
};
use bevy::prelude::*;

pub(crate) struct ClientMapPlug;

impl Plugin for ClientMapPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, render_map.after(init_map_tmp));
    }
}

pub(crate) fn render_map(
    mut commands: Commands,
    map: Res<Map>,
    query: Query<&SpriteName>,
    sprites: Res<Sprites>,
) {
    let render_entity = |commands: &mut Commands, (x, y), ent| {
        commands.entity(ent).insert(Transform {
            translation: Vec3::new(x as f32, y as f32, 0.0),
            ..Default::default()
        });
        if let Some(sprite) = query
            .get(ent)
            .ok()
            .and_then(|sprite_name| sprites.0.get(&sprite_name.0))
        {
            commands.entity(ent).insert(sprite.clone());
        }
    };

    map.floor
        .iter()
        .for_each(|(&(x, y), ent)| render_entity(&mut commands, (x, y), *ent));
    map.wall
        .iter()
        .for_each(|(&(x, y), ent)| render_entity(&mut commands, (x, y), *ent));
}
