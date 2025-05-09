use crate::shared::sprites::SpriteName;
use bevy::prelude::*;
use std::collections::HashMap;

pub(crate) struct SharedMapPlug;
impl Plugin for SharedMapPlug {
    fn build(&self, app: &mut App) {
        app.init_resource::<Map>();
        app.add_systems(Startup, load_map_tmp);
        app.add_systems(Startup, init.after(load_map_tmp));
    }
}

type Floor = HashMap<(i32, i32), Entity>;
type Wall = HashMap<(i32, i32), Entity>;
type StationaryObjects = HashMap<(i32, i32), Entity>;
type Entities = Vec<Entity>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Floor,
    Wall,
    Stationary,
    Entity,
}

#[derive(Resource, Debug, Clone, Default)]
pub(crate) struct Map {
    pub floor: Floor,
    pub wall: Wall,
    #[allow(dead_code)]
    pub stationary_objects: StationaryObjects,
    #[allow(dead_code)]
    pub entities: Entities,
}

pub struct MapIterator {
    floor_iter: std::collections::hash_map::IntoValues<(i32, i32), Entity>,
    wall_iter: std::collections::hash_map::IntoValues<(i32, i32), Entity>,
    stationary_iter: std::collections::hash_map::IntoValues<(i32, i32), Entity>,
    entities_iter: std::vec::IntoIter<Entity>,
    current_layer: Layer,
}

impl MapIterator {
    pub fn new(map: Map) -> Self {
        Self {
            floor_iter: map.floor.into_values(),
            wall_iter: map.wall.into_values(),
            stationary_iter: map.stationary_objects.into_values(),
            entities_iter: map.entities.into_iter(),
            current_layer: Layer::Floor,
        }
    }
    #[allow(dead_code)]
    pub fn with_layer(self) -> MapLayerIterator {
        MapLayerIterator { inner: self }
    }

    pub fn current_layer(&self) -> Layer {
        self.current_layer
    }
}

impl Iterator for MapIterator {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.current_layer {
                Layer::Floor => {
                    if let Some(entity) = self.floor_iter.next() {
                        return Some(entity);
                    } else {
                        self.current_layer = Layer::Wall;
                    }
                }
                Layer::Wall => {
                    if let Some(entity) = self.wall_iter.next() {
                        return Some(entity);
                    } else {
                        self.current_layer = Layer::Stationary;
                    }
                }
                Layer::Stationary => {
                    if let Some(entity) = self.stationary_iter.next() {
                        return Some(entity);
                    } else {
                        self.current_layer = Layer::Entity;
                    }
                }
                Layer::Entity => {
                    return self.entities_iter.next();
                }
            }
        }
    }
}

pub struct MapLayerIterator {
    inner: MapIterator,
}

impl Iterator for MapLayerIterator {
    type Item = (Layer, Entity);

    fn next(&mut self) -> Option<Self::Item> {
        let layer = self.inner.current_layer();
        self.inner.next().map(|entity| (layer, entity))
    }
}

impl IntoIterator for Map {
    type Item = Entity;
    type IntoIter = MapIterator;

    fn into_iter(self) -> Self::IntoIter {
        MapIterator::new(self)
    }
}

pub struct MapRefIterator<'a> {
    floor_iter: std::collections::hash_map::Values<'a, (i32, i32), Entity>,
    wall_iter: std::collections::hash_map::Values<'a, (i32, i32), Entity>,
    stationary_iter: std::collections::hash_map::Values<'a, (i32, i32), Entity>,
    entities_iter: std::slice::Iter<'a, Entity>,
    current_layer: Layer,
}

impl<'a> MapRefIterator<'a> {
    pub(crate) fn new(map: &'a Map) -> Self {
        Self {
            floor_iter: map.floor.values(),
            wall_iter: map.wall.values(),
            stationary_iter: map.stationary_objects.values(),
            entities_iter: map.entities.iter(),
            current_layer: Layer::Floor,
        }
    }

    pub fn with_layer(self) -> MapLayerRefIterator<'a> {
        MapLayerRefIterator { inner: self }
    }

    pub fn current_layer(&self) -> Layer {
        self.current_layer
    }
}

impl<'a> Iterator for MapRefIterator<'a> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.current_layer {
                Layer::Floor => {
                    if let Some(entity) = self.floor_iter.next() {
                        return Some(entity);
                    } else {
                        self.current_layer = Layer::Wall;
                    }
                }
                Layer::Wall => {
                    if let Some(entity) = self.wall_iter.next() {
                        return Some(entity);
                    } else {
                        self.current_layer = Layer::Stationary;
                    }
                }
                Layer::Stationary => {
                    if let Some(entity) = self.stationary_iter.next() {
                        return Some(entity);
                    } else {
                        self.current_layer = Layer::Entity;
                    }
                }
                Layer::Entity => {
                    return self.entities_iter.next();
                }
            }
        }
    }
}

pub struct MapLayerRefIterator<'a> {
    inner: MapRefIterator<'a>,
}

impl<'a> MapLayerRefIterator<'a> {
    pub(crate) fn new(map: &'a Map) -> Self {
        MapLayerRefIterator {
            inner: MapRefIterator::new(map),
        }
    }
}

impl<'a> Iterator for MapLayerRefIterator<'a> {
    type Item = (Layer, &'a Entity);

    fn next(&mut self) -> Option<Self::Item> {
        let layer = self.inner.current_layer();
        self.inner.next().map(|entity| (layer, entity))
    }
}

impl Map {
    pub fn iter(&self) -> MapRefIterator {
        MapRefIterator::new(self)
    }
    #[allow(dead_code)]
    pub fn iter_with_layer(&self) -> MapLayerRefIterator {
        MapLayerRefIterator::new(self)
    }

    #[allow(dead_code)]
    pub fn into_iter(self) -> MapIterator {
        MapIterator::new(self)
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = &'a Entity;
    type IntoIter = MapRefIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, Component)]
pub(crate) struct MapObject;

#[derive(Debug, Component)]
pub(crate) struct MapFloor;

#[derive(Debug, Component)]
pub(crate) struct MapWall;

#[derive(Debug, Component)]
pub(crate) struct MapStationary;

#[derive(Debug, Component)]
pub(crate) struct MapEntity;

pub(crate) fn load_map_tmp(mut commands: Commands, mut map: ResMut<Map>) {
    //tempory system to spawn
    //simple sqr
    //just wait when
    //LightVillet make
    //the map editor
    //I guess here we will load map from file to the game.
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

pub(crate) fn init(mut commands: Commands, map: Res<Map>) {
    map.into_iter().with_layer().for_each(|(layer, ent)| {
        commands.entity(*ent).insert(MapObject);
        match layer {
            Layer::Floor => commands.entity(*ent).insert(MapFloor),
            Layer::Wall => commands.entity(*ent).insert(MapWall),
            Layer::Stationary => commands.entity(*ent).insert(MapStationary),
            Layer::Entity => commands.entity(*ent).insert(MapEntity),
        };
    });
}
