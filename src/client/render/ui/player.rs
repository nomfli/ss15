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
pub(crate) struct HandsUI;

pub(crate) fn init_hands_ui(query: Query<Entity, With<UIRoot>>, mut commands: Commands) {
    let Some(node_ent) = make_log!(query.single(), "init hands ui can't, get root node") else {
        return;
    };
    println!("FUUUUUUUCK");
    commands.entity(node_ent).with_children(|parent| {
        parent.spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::WHITE),
            BorderRadius::MAX,
            BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
        ));
    });
}
