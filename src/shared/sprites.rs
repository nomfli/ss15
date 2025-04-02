use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct SpritesPlug;

impl Plugin for SpritesPlug {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sprites>();
        app.add_systems(Startup, init_sprites);
    }
}

#[derive(Debug, Default, Resource)]
pub struct Sprites(pub HashMap<String, Sprite>);

#[derive(Debug, Default, Component, Deserialize, Serialize, Clone)]
pub struct SpriteName(pub String);

pub fn init_sprites(mut sprites: ResMut<Sprites>, asset_server: Res<AssetServer>) {
    let red_sqr = "red_sqr".to_string();
    sprites.0.insert(
        red_sqr,
        Sprite {
            custom_size: Some(Vec2::new(32.0, 32.0)),
            color: Color::srgb_u8(255, 0, 0),
            ..Default::default()
        },
    );
    let adrenalin = "adrenalin".to_string();

    sprites.0.insert(
        adrenalin,
        Sprite::from_image(asset_server.load("./images/adrenalin.png")),
    );
}
