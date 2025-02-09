use bevy::prelude::*;
mod movement;
use movement::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_movement)
        .add_systems(Update, velocity)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        #[warn(deprecated)]
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Speed { x: 0.0, y: 0.0 },
        MaxSpeed(400.0),
        Acceleration(50.0),
    ));
}
