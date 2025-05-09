use crate::shared::{
    components::Grabbable,
    sprites::{init_sprites, SpriteName, Sprites},
};
use bevy::prelude::*;

pub struct ServerInitPlug;

impl Plugin for ServerInitPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.after(init_sprites));
    }
}

pub(crate) fn init(sprites: Res<Sprites>, mut commands: Commands) {
    let name = "adrenalin".to_string();
    let Some(sprite) = sprites.0.get(&name) else {
        panic!("Expected sprite '{}' not found in sprites resource", name)
    };
    commands
        .spawn(sprite.clone())
        .insert(SpriteName(name))
        .insert(Transform {
            translation: Vec3::new(100.0, 100.0, 0.0),
            ..Default::default()
        })
        .insert(Grabbable(true));
}
