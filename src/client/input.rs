use crate::shared::resource::MovementInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct InputClientPlug;

impl Plugin for InputClientPlug {
    fn build(&self, app: &mut App) {
        app.init_resource::<Mouse>();
        app.init_resource::<ThrowInput>();
        app.add_systems(Update, mouse_coords_to_world);
        app.add_systems(Update, (movement_input, throw_input));
    }
}

#[derive(Clone, Copy, Default, Debug, Resource)]
pub struct ThrowInput(pub bool);

#[derive(Clone, Copy, Default, Debug, Resource)]
pub struct Mouse {
    pub cords: Option<Vec2>,
    pub left_button: bool,
}

pub fn mouse_coords_to_world(
    mut mouse: ResMut<Mouse>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    mouse.left_button = mouse_button.pressed(MouseButton::Left);
    let window = q_windows.single();
    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok((camera, camera_transform)) = q_camera.get_single() {
            match camera.viewport_to_world(camera_transform, cursor_pos) {
                Ok(ray) => {
                    mouse.cords = Some(ray.origin.truncate());
                }
                Err(_) => {
                    mouse.cords = None;
                }
            }
        }
    } else {
        mouse.cords = None;
    }
}

pub(crate) fn movement_input(
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

pub(crate) fn throw_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut throw_input: ResMut<ThrowInput>,
) {
    throw_input.0 = keyboard_input.pressed(KeyCode::KeyQ);
}
