use crate::{
    client::render::{
        connection::player_connected,
        init::{init_ui_root, UIRoot},
    },
    make_log,
    shared::{
        components::{Hands, PlayerEntity},
        utils::Loggable,
    },
};
use bevy::prelude::*;

pub(crate) struct UIHandsPlug;

impl Plugin for UIHandsPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            init_hands_ui.after(player_connected).after(init_ui_root),
        );
    }
}

#[derive(Component)]
pub(crate) struct HandsUI(pub usize);

pub(crate) fn init_hands_ui(
    query: Query<Entity, With<UIRoot>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Some(node_ent) = make_log!(query.single(), "init hands ui can't, get root node") else {
        return;
    };

    let texture: Handle<Image> = asset_server.load("./images/hands.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 2, 2, None, None);
    let handle_layout = texture_atlases.add(layout);

    commands.entity(node_ent).with_children(|parent| {
        parent.spawn((
            Button,
            Node {
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                border: UiRect::all(Val::Px(5.0)),
                bottom: Val::Px(20.0),
                left: Val::Percent(47.0),
                position_type: PositionType::Absolute,
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: handle_layout.clone(),
                    index: 2,
                },
            ),
            HandsUI(0),
        ));
    });
    commands.entity(node_ent).with_children(|parent| {
        parent.spawn((
            Button,
            Node {
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                border: UiRect::all(Val::Px(5.0)),
                bottom: Val::Px(20.0),
                left: Val::Percent(53.0),
                position_type: PositionType::Absolute,
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: handle_layout,
                    index: 0,
                },
            ),
            HandsUI(0),
        ));
    });
}
