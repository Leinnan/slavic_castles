use std::time::Duration;

use super::game::{OpponentInformation, PlayerInformation};
use super::{game::NamesAsset, game_states::GameState};
use crate::data::{deck::DeckAsset, profile};
use crate::helpers::despawn_recursive_by_component;
use crate::states::consts::*;
use bevy::prelude::*;
use bevy_button_released_plugin::ButtonReleasedEvent;
// use bevy_ecss::prelude::{Class, StyleSheet};
use bevy_pkv::PkvStore;
use bevy_tweening::{lens::TransformScaleLens, Animator, Delay, EaseFunction, Tween};
use rand::{thread_rng, Rng};

#[derive(Component)]
pub enum MainMenuButton {
    StartGame,
    EditProfile,
    OpenRepository,
    Exit,
}
use super::consts;

#[derive(Component)]
pub struct MenuObject;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), (check_for_profile, setup_menu))
            .add_systems(Update, button_system.run_if(in_state(GameState::Menu)))
            .add_systems(
                OnExit(GameState::Menu),
                despawn_recursive_by_component::<MenuObject>,
            );
    }
}

fn check_for_profile(
    mut next_state: ResMut<NextState<GameState>>,
    pkv: Res<PkvStore>,
    deck: Res<Assets<DeckAsset>>,
    names: Res<Assets<NamesAsset>>,
    mut commands: Commands,
) {
    let Some(profile) = profile::get_profile(&pkv) else {
        next_state.set(GameState::ProfileEdit);
        return;
    };
    let Some(deck_asset) = deck.iter().next() else {
        panic!("NO DECK ASSET");
    };
    let Some(name_asset) = names.iter().next() else {
        panic!("NO NAMES ASSET");
    };
    commands.insert_resource(PlayerInformation {
        name: profile.name.clone(),
        avatar_id: profile.avatar_id,
        deck: deck_asset.1.clone(),
        ..Default::default()
    });
    let mut rng = thread_rng();
    let avatar_id: i32 = 1 + (rng.gen::<i32>().abs() % (AVATARS_AMOUNT - 1));
    commands.insert_resource(OpponentInformation(PlayerInformation {
        name: name_asset.1.get_random(),
        deck: deck_asset.1.clone(),
        avatar_id,
        ..Default::default()
    }));
}

fn button_system(
    mut reader: EventReader<ButtonReleasedEvent>,
    interaction_query: Query<&MainMenuButton>,
    mut next_state: ResMut<NextState<GameState>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    for event in reader.read() {
        if let Ok(button_type) = interaction_query.get(**event) {
            match *button_type {
                MainMenuButton::StartGame => {
                    next_state.set(GameState::Game);
                }
                MainMenuButton::OpenRepository => {
                    webbrowser::open(env!("CARGO_PKG_HOMEPAGE")).unwrap_or_default()
                }
                MainMenuButton::EditProfile => next_state.set(GameState::ProfileEdit),
                MainMenuButton::Exit => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        //
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        exit.send(bevy::app::AppExit::Success);
                    }
                }
            }
        }
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                height: FULL_SIZE_PERCENT,
                width: FULL_SIZE_PERCENT,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(MenuObject)
        .insert(Name::new("menu-root"))
        // .insert(StyleSheet::new(asset_server.load("css/base.css")))
        .with_children(|parent| {
            let init_scale = Vec3::splat(0.01);
            parent.spawn(ImageBundle {
                z_index: ZIndex::Global(-1),
                image: UiImage {
                    texture: asset_server.load("img/start_screen_bg.png"),
                    ..default()
                },
                ..default()
            });
            // .insert(Class::new("menu_background"));
            parent.spawn((
                ImageBundle {
                    image: UiImage {
                        texture: asset_server.load("img/logo.png"),
                        ..default()
                    },
                    transform: Transform::from_scale(init_scale),
                    style: Style {
                        margin: UiRect {
                            top: Val::Percent(5.0),
                            bottom: Val::Auto,
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
                Animator::new(Tween::new(
                    EaseFunction::BounceOut,
                    Duration::from_millis(900),
                    TransformScaleLens {
                        start: init_scale,
                        end: Vec3::ONE,
                    },
                )),
            ));
            // .insert(Class::new("logo"));

            let img_style = TextStyle {
                font: asset_server.load(consts::LABEL_FONT),
                font_size: 30.0,
                color: Color::linear_rgb(0.7, 0.7, 0.7),
            };
            let mut start_time_ms = 500;

            for (text, label, margin) in [
                (
                    "Quick Fight",
                    MainMenuButton::StartGame,
                    UiRect {
                        top: Val::Auto,
                        bottom: Val::Px(15.0),
                        ..default()
                    },
                ),
                (
                    "Edit Profile",
                    MainMenuButton::EditProfile,
                    UiRect {
                        bottom: Val::Px(15.0),
                        ..default()
                    },
                ),
                #[cfg(not(target_arch = "wasm32"))]
                (
                    "Exit Game",
                    MainMenuButton::Exit,
                    UiRect {
                        bottom: Val::Px(15.0),
                        ..default()
                    },
                ),
            ] {
                let tween_scale = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(500),
                    TransformScaleLens {
                        start: init_scale,
                        end: Vec3::ONE,
                    },
                );

                let animator = if start_time_ms > 0 {
                    let delay = Delay::new(Duration::from_millis(start_time_ms));
                    Animator::new(delay.then(tween_scale))
                } else {
                    Animator::new(tween_scale)
                };

                start_time_ms += 200;

                parent
                    .spawn((
                        ButtonBundle {
                            image: asset_server.load("img/panel-006.png").into(),
                            style: Style {
                                margin,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            transform: Transform::from_scale(init_scale),
                            background_color: Srgba::hex("7A444A").unwrap().into(),
                            ..default()
                        },
                        ImageScaleMode::Sliced(TextureSlicer {
                            border: BorderRect::square(29.0),
                            center_scale_mode: SliceScaleMode::Stretch,
                            sides_scale_mode: SliceScaleMode::Stretch,
                            max_corner_scale: 1.0,
                        }),
                        // Class::new("menu common"),
                        Name::new(format!("button:{}", text)),
                        animator,
                        label,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(text, img_style.clone()));
                    });
            }
            parent.spawn((
                TextBundle::from_section(
                    format!("v.{}\n{}", VERSION, GIT_HASH),
                    TextStyle {
                        font: asset_server.load(REGULAR_FONT),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Px(15.0),
                        top: Val::Auto,
                        bottom: Val::Px(15.0),
                    },
                    position_type: PositionType::Absolute,
                    ..default()
                })
                .with_text_justify(JustifyText::Right),
                Interaction::None,
                MainMenuButton::OpenRepository,
            ));
        });
}
