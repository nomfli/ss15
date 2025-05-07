use crate::shared::sprites::SpriteName;
use bevy::prelude::*;
use std::collections::HashMap;

pub(crate) struct SharedMapPlug;
impl Plugin for SharedMapPlug {
    fn build(&self, app: &mut App) {
        app.init_resource::<Map>();
        app.add_systems(Startup, init_map_tmp);
    }
}

type Floor = HashMap<(i32, i32), Entity>; // We can't use HashMap 'cause rust don't have hash impletation
                                          // for floats, but we can use crate with another ordered float
                                          // realization
type Wall = HashMap<(i32, i32), Entity>;
type StationaryObjects = HashMap<(i32, i32), Entity>;
type Entities = Vec<Entity>;

#[derive(Resource, Debug, Clone, Default)]
pub(crate) struct Map {
    pub floor: Floor,
    pub wall: Wall,
    pub stationary_objects: StationaryObjects,
    pub entities: Entities,
}

pub(crate) fn init_map_tmp(mut commands: Commands, mut map: ResMut<Map>) {
    //tempory system to spawn
    //simple sqr
    //just wait when
    //LightVillet make
    //the map editor
    for i in 0..5 {
        for j in 0..5 {
            let coords = ((i * 64), (j * 64));
            if j == 0 || i == 0 || i == 5 || j == 5 {
                let ent = commands.spawn(SpriteName("simple_wall".to_string())).id();
                map.wall.insert(coords, ent);
            } else {
                let ent = commands.spawn(SpriteName("simple_floor".to_string())).id();
                map.floor.insert(coords, ent);
            }
        }
    }
}
