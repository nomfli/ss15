use crate::{
    client::render::{
        connection::player_connected,
        init::{init_ui_root, UIRoot},
    },
    make_log,
    shared::{
        components::{Hands, PlayerEntity},
        sprites::{SpriteName, Sprites},
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
        app.add_systems(Update, update_hands_ui.after(init_ui_root));
        app.add_systems(Update, handle_grabbed_items_ui.after(init_ui_root));
    }
}

#[derive(Component)]
pub(crate) struct HandsUI(pub usize);

#[derive(Component)]
pub(crate) struct GrabbedObjUI(pub usize);

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

    let left = commands
        .entity(node_ent)
        .with_children(|parent| {
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
        })
        .id();

    let right = commands
        .entity(node_ent)
        .with_children(|parent| {
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
                HandsUI(1),
            ));
        })
        .id();

    commands.entity(right).with_children(|parent| {
        parent.spawn((
            Button,
            Node {
                width: Val::Px(25.0),
                height: Val::Px(25.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                bottom: Val::Px(33.0),
                left: Val::Percent(54.0),
                ..default()
            },
            GrabbedObjUI(1),
        ));
    });
    commands.entity(left).with_children(|parent| {
        parent.spawn((
            Button,
            Node {
                width: Val::Px(25.0),
                height: Val::Px(25.0),
                border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                bottom: Val::Px(33.0),
                left: Val::Percent(48.0),
                ..default()
            },
            GrabbedObjUI(0),
        ));
    });
}

pub(crate) fn update_hands_ui(
    mut ui_query: Query<(&HandsUI, &mut ImageNode)>,
    player_q: Query<&Hands, With<PlayerEntity>>,
) {
    for (ui, mut node) in ui_query.iter_mut() {
        if let Ok(hands) = player_q.single() {
            if let Some(atlas) = &mut node.texture_atlas {
                if hands.selected_hand == ui.0 {
                    if atlas.index == 0 {
                        atlas.index = 1;
                    }
                    if atlas.index == 2 {
                        atlas.index = 3;
                    }
                } else {
                    if atlas.index == 1 {
                        atlas.index = 0;
                    }
                    if atlas.index == 3 {
                        atlas.index = 2;
                    }
                }
            }
        }
    }
}

pub(crate) fn handle_grabbed_items_ui(
    ui_q: Query<(Entity, &GrabbedObjUI), With<Node>>,
    player: Query<&Hands, With<PlayerEntity>>,
    sprite_q: Query<&SpriteName>,
    sprites: Res<Sprites>,
    mut commands: Commands,
) {
    let Ok(hands) = player.single() else {
        return;
    };

    let selected_hand = hands.selected_hand;

    for (ent, ui) in ui_q.iter() {
        if ui.0 != selected_hand {
            continue;
        }

        let Some(grabbed_ent) = hands.all_hands[selected_hand].grab_ent else {
            commands.entity(ent).remove::<ImageNode>();
            continue;
        };

        if let Ok(sprite_name) = sprite_q.get(grabbed_ent) {
            if let Some(sprite) = sprites.0.get(&sprite_name.0) {
                commands.entity(ent).insert(ImageNode {
                    image: sprite.image.clone(),
                    texture_atlas: sprite.texture_atlas.clone(),
                    ..Default::default()
                });
            }
        }
    }
}
