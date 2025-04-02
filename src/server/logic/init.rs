use crate::shared::{
    components::Grabbable,
    sprites::{SpriteName, Sprites},
};
use bevy::prelude::*;

pub(crate) fn init(sprites: Res<Sprites>, mut commands: Commands) {
    let name = "blue_sqr".to_string();
    let Some(sprite) = sprites.0.get(&name) else {
        panic!("Expected sprite '{}' not found in sprites resource", name)
    };
    commands
        .spawn(sprite.clone())
        .insert(SpriteName(name))
        .insert(Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Grabbable(true));
}
