use bevy::prelude::*;

pub(crate) struct InitRenderPlug;

impl Plugin for InitRenderPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_camera);
        app.add_systems(Startup, init_ui_root);
    }
}

pub(crate) fn start_camera(mut commands: Commands) {
    commands.spawn(Camera2d {});
}

#[derive(Component)]
pub(crate) struct UIRoot;

pub(crate) fn init_ui_root(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        UIRoot,
    ));
}
