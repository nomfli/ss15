use bevy::{prelude::*, render::view::RenderLayers};

pub(crate) struct InitRenderPlug;

impl Plugin for InitRenderPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_camera);
    }
}

pub(crate) fn start_camera(mut commands: Commands) {
    commands
        .spawn(Camera2d {})
        .insert(RenderLayers::from_layers(&(0..32).collect::<Vec<_>>()));
}
