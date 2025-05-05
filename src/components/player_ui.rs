use bevy::{ecs::query::QueryData, prelude::*, reflect::Reflect};
use game_core::data::resource::ResourceType;
use serde::{Deserialize, Serialize};

use crate::states::{
    consts,
    game::{self, GameObject, PlayerNumber, PlayerQueryItem, Players, PlayersUpdated},
    game_states::GameState,
};

#[derive(Component, Default, Reflect, Serialize, Deserialize, Debug, Copy, Clone)]
#[require(PlayerUi, PlayerUiValue)]
pub enum PlayerTextInterface {
    ResourceAmount(ResourceType),
    ResourceProduction(ResourceType),
    Health,
    #[default]
    Shield,
}

trait PlayerInterfaceHelper {
    fn player_ui(&self, element: PlayerTextInterface, player: PlayerNumber) -> (PlayerTextInterface, PlayerUi, TextFont);
}

impl PlayerInterfaceHelper for Res<'_, AssetServer> {
    fn player_ui(&self, element: PlayerTextInterface, player: PlayerNumber) -> (PlayerTextInterface, PlayerUi, TextFont) {
        let text_font =
            TextFont::from_font(self.load(consts::REGULAR_FONT)).with_font_size(20.0);

        (element, PlayerUi(player), text_font)
    }
}

#[derive(Component, Default, Reflect, Serialize, Deserialize, Debug)]
#[require(Text)]
pub struct PlayerUiValue(pub i32);

#[derive(Hash, Ord, PartialOrd, PartialEq, Eq, Default, Debug)]
pub enum UpdateResult{
    #[default]
    NoChange,
    BiggerValue,
    SmallerValue
}

impl PlayerUiValue {
    fn update(&mut self, new: i32) -> UpdateResult {
        if new.eq(&self.0) {
            return UpdateResult::NoChange;
        }
        let result = if new > self.0 { UpdateResult::BiggerValue } else { UpdateResult::SmallerValue };
        self.0 = new;
        result
    }
}

#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct TextUiPlayerElements {
    target: &'static PlayerUi,
    element: &'static PlayerTextInterface,
    value: &'static mut PlayerUiValue,
    text: &'static mut Text,
}

impl TextUiPlayerElementsItem<'_> {
    pub fn update(&mut self, data: &PlayerQueryItem) -> UpdateResult {
        let new_value = match &self.element
        {
            PlayerTextInterface::ResourceAmount(res_type) => {
                data.supply.get(*res_type).amount
            }
            PlayerTextInterface::ResourceProduction(res_type) => {
                data.supply.get(*res_type).production
            }
            PlayerTextInterface::Health => data.player.tower_hp,
            PlayerTextInterface::Shield => data.player.walls_hp,
        };
        let result = self.value.update(new_value);
        if &result == &UpdateResult::NoChange {
            return result;
        }
        self.text.0 = match &self.element {
            PlayerTextInterface::ResourceProduction(_) if self.value.0 > 0 => {
                format!("+{}", self.value.0)
            }
            _ => self.value.0.to_string()
        };
        result
    }
}

pub struct PlayerUiPlugin;

impl Plugin for PlayerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_created_player_ui, update_player_ui).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            OnEnter(GameState::Game),
            (setup_player_ui).after(game::init_players),
        );
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Deref, Default, Debug)]
pub struct PlayerUi(pub PlayerNumber);

fn update_created_player_ui(
    mut ui_query: Query<TextUiPlayerElements, Added<PlayerTextInterface>>,
    player_query: Players,
) {
    for mut el in ui_query.iter_mut() {
        if let Some(player) = player_query.get_player(el.target.0) {
            el.update(&player);
        }
    }
}

pub fn update_player_ui(mut ui_query: Query<TextUiPlayerElements>, player_query: PlayersUpdated) {
    for player in player_query.iter() {
        for mut el in ui_query.iter_mut() {
            if !el.target.0.eq(player.nr) {
                continue;
            }
            el.update(&player);
        }
    }
}

fn setup_player_ui(mut commands: Commands, asset_server: Res<AssetServer>, player_query: Players) {
    let header_style =
        TextFont::from_font(asset_server.load(consts::LABEL_FONT)).with_font_size(30.0);

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
        let player_info = player_query.get_player(player).expect("ERROR");
        let avatar_path = player_info.details.avatar_path();
        commands
            .spawn((style.clone(), GameObject))
            .insert(Name::new(format!("Ui{:?}", player)))
            .with_children(|p| {
                p.spawn((
                    ImageNode {
                        image: asset_server.load(avatar_path),
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
                            Text::new(player_info.details.name.clone()),
                            header_style.clone(),
                            TextLayout::new_with_justify(if right_align {
                                JustifyText::Right
                            } else {
                                JustifyText::Left
                            }),
                        ))
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
                    res.spawn(asset_server.player_ui(PlayerTextInterface::Health, player));
                    res.spawn(ImageNode::new(asset_server.load("img/player_shield.png")));
                    res.spawn(asset_server.player_ui(PlayerTextInterface::Shield, player));
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
                        .with_child(asset_server.player_ui(PlayerTextInterface::ResourceAmount(resource),player));
                        p.spawn(Node {
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(3.0),
                            left: Val::Px(3.0),
                            width: Val::Px(30.0),
                            height: Val::Px(30.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        })
                        .with_child(asset_server.player_ui(PlayerTextInterface::ResourceProduction(resource),player));
                    });
                }
            });
    }
}
