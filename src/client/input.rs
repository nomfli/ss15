use crate::shared::resource::MovementInput;
use bevy::prelude::*;
pub struct InputClientPlug;

impl Plugin for InputClientPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_input);
    }
}

pub fn movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_input: ResMut<MovementInput>,
) {
    player_input.left =
        keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    player_input.right =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
    player_input.up =
        keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    player_input.down =
        keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
}
