use bevy::{prelude::*, reflect::Reflect};
use serde::{Deserialize, Serialize};

use crate::{
    data::{player::Player, player_resources::PlayerResources, resource::ResourceType},
    states::{
        consts,
        game::{self, AvatarId, GameObject, PlayerNumber},
        game_states::GameState,
    },
};

#[derive(Component, Default, Reflect, Serialize, Deserialize)]
pub enum PlayerUiElement {
    ResourceAmount(ResourceType),
    ResourceProduction(ResourceType),
    Health,
    Shield,
    Avatar,
    #[default]
    Name,
}

pub struct PlayerUiPlugin;

impl Plugin for PlayerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_player_ui).run_if(in_state(GameState::Game)))
            .add_systems(
                OnEnter(GameState::Game),
                (setup_player_ui).after(game::init_players),
            );
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Deref)]
pub struct PlayerUi(pub PlayerNumber);

pub fn update_player_ui(
    mut ui_query: Query<(&PlayerUiElement, &mut Text, &PlayerUi)>,
    player_query: Query<(&Player, &PlayerNumber, &PlayerResources, &Name, &AvatarId)>,
) {
    for (element, mut text, player) in ui_query.iter_mut() {
        let Some((player, _, resources, name, _avatar_id)) =
            player_query.iter().find(|el| el.1 == &**player)
        else {
            continue;
        };
        **text = match element {
            PlayerUiElement::ResourceAmount(res) => resources.get(*res).amount.to_string(),
            PlayerUiElement::ResourceProduction(res) => {
                let prod = resources.get(*res).production;
                let sign = if prod > 0 { "+" } else { "" };
                format!("{}{}", sign, prod)
            }
            PlayerUiElement::Health => player.tower_hp.to_string(),
            PlayerUiElement::Shield => player.walls_hp.to_string(),
            PlayerUiElement::Name => name.to_string(),
            PlayerUiElement::Avatar => todo!(),
        };
    }
}

fn setup_player_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<(&PlayerNumber, &AvatarId)>,
) {
    let img_style =
        TextFont::from_font(asset_server.load(consts::REGULAR_FONT)).with_font_size(25.0);

    // let img_style = TextStyle {
    //     font: asset_server.load(consts::REGULAR_FONT),
    //     font_size: 25.0,
    //     color: Color::linear_rgb(0.9, 0.9, 0.9),
    // };
    let header_style =
        TextFont::from_font(asset_server.load(consts::LABEL_FONT)).with_font_size(30.0);

    // let header_style = TextStyle {
    //     font: asset_server.load(consts::LABEL_FONT),
    //     font_size: 30.0,
    //     // color: Color::GOLD,
    //     ..default()
    // };
    for (player, style, right_align) in [
        (
            PlayerNumber::First,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(15.0),
                margin: UiRect::all(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(15.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            false,
        ),
        (
            PlayerNumber::Second,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(15.0),
                margin: UiRect::all(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(15.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            true,
        ),
    ] {
        let avatar = player_query.iter().find(|e| e.0.eq(&player)).unwrap();
        commands
            .spawn((style.clone(), GameObject))
            .insert(Name::new(format!("Ui{:?}", player)))
            .with_children(|p| {
                p.spawn((
                    ImageNode {
                        image: asset_server.load(crate::data::profile::get_avatar_path(**avatar.1)),
                        ..default()
                    },
                    Node {
                        width: Val::Px(128.0),
                        height: Val::Px(128.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        ImageNode {
                            image: asset_server.load("img/player_frame_name.png"),
                            flip_x: right_align,
                            ..default()
                        },
                        Node {
                            padding: UiRect::all(Val::Px(8.0)),
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            ..default()
                        },
                    ))
                    .with_children(|name| {
                        name.spawn((
                            Text::new("TEST"),
                            header_style.clone(),
                            TextLayout::new_with_justify(if right_align {
                                JustifyText::Right
                            } else {
                                JustifyText::Left
                            }),
                        ))
                        .insert(PlayerUiElement::Name)
                        .insert(PlayerUi(player));
                    });
                });
                p.spawn((
                    ImageNode::new(asset_server.load("img/player_frame_resources.png")),
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_content: AlignContent::SpaceAround,
                        padding: UiRect::all(Val::Px(8.0)),
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                ))
                .with_children(|res| {
                    res.spawn(ImageNode::new(asset_server.load("img/player_health.png")));
                    res.spawn((Text::new("TEST"), img_style.clone()))
                        .insert(PlayerUiElement::Health)
                        .insert(PlayerUi(player));
                    res.spawn(ImageNode::new(asset_server.load("img/player_shield.png")));
                    res.spawn((Text::new("TEST"), img_style.clone()))
                        .insert(PlayerUiElement::Shield)
                        .insert(PlayerUi(player));
                });
                for (resource, gfx) in [
                    (ResourceType::Magic, "potionBlue"),
                    (ResourceType::Tools, "tools"),
                    (ResourceType::Soldiers, "axe"),
                ] {
                    let base_color = match &resource {
                        ResourceType::Tools => "#2a9efe",
                        ResourceType::Magic => "#339820",
                        ResourceType::Soldiers => "#bb332a",
                    };
                    let color = Srgba::hex(base_color).unwrap();

                    p.spawn((
                        ImageNode {
                            image: asset_server.load(format!("img/{}.png", gfx)),
                            color: color.into(),
                            ..default()
                        },
                        Node {
                            width: Val::Percent(100.0),
                            margin: UiRect::bottom(Val::Px(5.0)),
                            ..default()
                        },
                    ))
                    .with_children(|p| {
                        p.spawn(ImageNode::new(asset_server.load("img/resource_frame.png")));
                        p.spawn(Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(3.0),
                            left: Val::Px(3.0),
                            width: Val::Px(30.0),
                            height: Val::Px(30.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        })
                        .with_child((
                            Text::new("0"),
                            img_style.clone(),
                            TextLayout::new_with_justify(JustifyText::Center),
                            PlayerUi(player),
                            PlayerUiElement::ResourceAmount(resource),
                        ));
                        p.spawn(Node {
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(3.0),
                            left: Val::Px(3.0),
                            width: Val::Px(30.0),
                            height: Val::Px(30.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        })
                        .with_child((
                            Text::new("0"),
                            img_style.clone(),
                            TextLayout::new_with_justify(JustifyText::Center),
                            PlayerUi(player),
                            PlayerUiElement::ResourceProduction(resource),
                        ));
                    });
                }
            });
    }
}
