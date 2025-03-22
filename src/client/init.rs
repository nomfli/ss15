use crate::shared::Data;
use bevy::prelude::*;

pub fn startup(mut commands: Commands) {
    commands.spawn(Camera2d { ..default() });
}

pub fn init_data(mut data: ResMut<Data>) {
    let red_sqr = Sprite {
        custom_size: Some(Vec2::new(100.0, 100.0)),
        color: Color::srgb(255.0, 0.0, 0.0),
        ..Default::default()
    };
    data.sprite.insert("red_sqr".to_string(), red_sqr);
}
