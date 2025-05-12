use crate::shared::{
    map::{init, load_map_tmp, Map},
    sprites::{SpriteName, Sprites},
};
use bevy::prelude::*;

pub(crate) struct ClientMapPlug;

impl Plugin for ClientMapPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, render_map.after(load_map_tmp).after(init));
    }
}

pub fn render_map(
    mut commands: Commands,
    map: Res<Map>,
    query: Query<(Entity, &SpriteName)>,
    sprites: Res<Sprites>,
) {
    map.iter()
        .filter_map(|entity| query.get(*entity).ok())
        .filter_map(|(ent, sprite_name)| {
            sprites
                .0
                .get(&sprite_name.0)
                .map(|sprite| (ent, sprite.clone()))
        })
        .for_each(|(entity, sprite)| {
            commands.entity(entity).insert(sprite);
        });
}
