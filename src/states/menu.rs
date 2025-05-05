use std::time::Duration;

use super::game::{OpponentInformation, PlayerInformation};
use super::{game::NamesAsset, game_states::GameState};
use crate::data::deck::DeckAsset;
use crate::data::profile::ProfileProvider;
use crate::helpers::button::ButtonReleased;
use crate::states::consts::*;
use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;
// use bevy_ecss::prelude::{Class, StyleSheet};
use bevy_pkv::PkvStore;
use bevy_tweening::{lens::TransformScaleLens, Animator, Delay, Tween};
use rand::{thread_rng, Rng};

use super::consts;


pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Menu),
            (check_for_profile, setup_menu).chain(),
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
    let Some(profile) = pkv.get_profile() else {
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

fn start_game(_: Trigger<ButtonReleased>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Game);
}

fn edit_profile_game(_: Trigger<ButtonReleased>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::ProfileEdit);
}

fn open_repo(_: Trigger<ButtonReleased>) {
    webbrowser::open(env!("CARGO_PKG_HOMEPAGE")).unwrap_or_default();
}

#[cfg(not(target_arch = "wasm32"))]
fn exit_game(_: Trigger<ButtonReleased>, mut exit: EventWriter<bevy::app::AppExit>) {
    exit.send(bevy::app::AppExit::Success);
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(super::root_node())
        .insert(StateScoped(GameState::Menu))
        .insert(Name::new("menu-root"))
        // .insert(StyleSheet::new(asset_server.load("css/base.css")))
        .with_children(|parent| {
            let init_scale = Vec3::splat(0.01);
            let bg = ImageNode::new(asset_server.load("img/start_screen_bg.png"));
            parent.spawn((bg, ZIndex(-1),
                         Node{
                             position_type: PositionType::Absolute,
                             top: Val::Px(0.0),
                             width: Val::Vw(100.0),
                             ..default()
                         }));
            // .insert(Class::new("menu_background"));
            parent.spawn((
                ImageNode::new(asset_server.load("img/logo.png")),
                Node {
                    margin: UiRect {
                        top: Val::Percent(5.0),
                        bottom: Val::Auto,
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

            let img_style = TextFont {
                font: asset_server.load(consts::LABEL_FONT),
                font_size: 30.0,
                ..default()
            };
            let clr = TextColor(Color::linear_rgb(0.7, 0.7, 0.7));
            let mut start_time_ms = 500;

            for (text, margin, observer) in [
                (
                    "Quick Fight",
                    UiRect {
                        top: Val::Auto,
                        bottom: Val::Px(15.0),
                        ..default()
                    },
                    Observer::new(start_game),
                ),
                (
                    "Edit Profile",
                    UiRect {
                        bottom: Val::Px(15.0),
                        ..default()
                    },
                    Observer::new(edit_profile_game),
                ),
                #[cfg(not(target_arch = "wasm32"))]
                (
                    "Exit Game",
                    UiRect {
                        bottom: Val::Px(15.0),
                        ..default()
                    },
                    Observer::new(exit_game),
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

                let id = parent
                    .spawn((
                        ImageNode {
                            image_mode: NodeImageMode::Sliced(TextureSlicer {
                                border: BorderRect::all(29.0),
                                center_scale_mode: SliceScaleMode::Stretch,
                                sides_scale_mode: SliceScaleMode::Stretch,
                                max_corner_scale: 1.0,
                            }),
                            color: Srgba::hex("7A444A").unwrap().into(),
                            image: asset_server.load("img/panel-006.png"),
                            ..Default::default()
                        },
                        Node {
                            margin,
                            padding: UiRect::all(Val::Px(15.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        Button,
                        // ButtonBundle {
                        //     image: asset_server.load("img/panel-006.png").into(),
                        //     style:
                        //     transform: Transform::from_scale(init_scale),
                        //     background_color: ,
                        //     ..default()
                        // },
                        // Class::new("menu common"),
                        Name::new(format!("button:{}", text)),
                        animator,
                    ))
                    .with_children(|parent| {
                        parent.spawn((Text::new(text), img_style.clone(), clr));
                    })
                    .id();
                let ob = observer.with_entity(id);
                parent.spawn(ob).set_parent(id);
            }
            let id = parent
                .spawn((
                    TextLayout::new_with_justify(JustifyText::Right),
                    Text(format!("v.{}\n{}", VERSION, GIT_HASH)),
                    TextColor(Color::WHITE),
                    TextFont {
                        font: asset_server.load(REGULAR_FONT),
                        font_size: 16.0,
                        ..default()
                    },
                    Node {
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Px(15.0),
                            top: Val::Auto,
                            bottom: Val::Px(15.0),
                        },
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    Interaction::None,
                    Button,
                ))
                .id();
            parent
                .spawn(Observer::new(open_repo).with_entity(id))
                .set_parent(id);
        });
}
