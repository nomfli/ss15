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

pub fn init_sprites(
    mut sprites: ResMut<Sprites>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let human = "human".to_string();
    let texture: Handle<Image> = asset_server.load("./images/human.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 2, 2, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);
    sprites.0.insert(
        human,
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index: 2,
            }),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()

        },
    );
    let adrenalin = "adrenalin".to_string();

    sprites.0.insert(
        adrenalin,
        Sprite::from_image(asset_server.load("./images/adrenalin.png")),
    );

    let texture_handle: Handle<Image> = asset_server.load("images/tiles.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let simple_wall = "simple_wall".to_string();
    sprites.0.insert(
        simple_wall,
        Sprite::from_atlas_image(
            texture_handle.clone(),
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 1,
            },
        ),
    );
    let simple_floor = "simple_floor".to_string();
    sprites.0.insert(
        simple_floor,
        Sprite::from_atlas_image(
            texture_handle,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
    );
}
