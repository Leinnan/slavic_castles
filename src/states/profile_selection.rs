use super::consts;
use super::game_states::GameState;
use crate::data::profile;
use crate::states::consts::*;
use bevy::prelude::*;
use bevy_button_released_plugin::ButtonReleasedEvent;
// use bevy_ecss::prelude::{Class, StyleSheet};
use bevy_pkv::PkvStore;
use bevy_simple_text_input::{TextInput, TextInputPlugin, TextInputSettings, TextInputValue};
use bevy_tweening::{lens::TransformScaleLens, Animator, Tween};
use rand::Rng;
use std::time::Duration;

#[derive(Resource)]
pub struct ProfileSelectionData {
    ui_entity: Entity,
}

pub struct ProfileSelectionPlugin;

#[derive(Component)]
pub enum ProfileEditButton {
    NextAvatar,
    PreviousAvatar,
    Save,
}

#[derive(Component, Reflect)]
pub struct AvatarDisplay {
    id: i32,
}

impl Plugin for ProfileSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextInputPlugin)
            .add_systems(OnEnter(GameState::ProfileEdit), setup_ui)
            .add_systems(
                Update,
                (avatar_update, button_system).run_if(in_state(GameState::ProfileEdit)),
            )
            .add_systems(OnExit(GameState::ProfileEdit), cleanup);
    }
}

fn avatar_update(
    mut avatar_query: Query<(&AvatarDisplay, &mut ImageNode), Changed<AvatarDisplay>>,
    asset_server: Res<AssetServer>,
) {
    for (avatar, mut image) in &mut avatar_query {
        let avatar_path = profile::get_avatar_path(avatar.id);
        image.image = asset_server.load(avatar_path);
    }
}

fn button_system(
    mut reader: EventReader<ButtonReleasedEvent>,
    interaction_query: Query<&ProfileEditButton>,
    mut next_state: ResMut<NextState<GameState>>,
    changed_value: Query<&TextInputValue>,
    mut avatar_query: Query<&mut AvatarDisplay>,
    mut pkv: ResMut<PkvStore>,
) {
    for event in reader.read() {
        if let Ok(button_type) = interaction_query.get(**event) {
            match *button_type {
                ProfileEditButton::NextAvatar => {
                    for mut avatar in &mut avatar_query {
                        avatar.id = if avatar.id + 1 > consts::AVATARS_AMOUNT {
                            1
                        } else {
                            avatar.id + 1
                        };
                    }
                }
                ProfileEditButton::PreviousAvatar => {
                    for mut avatar in &mut avatar_query {
                        avatar.id = if avatar.id - 1 < 1 {
                            consts::AVATARS_AMOUNT
                        } else {
                            avatar.id - 1
                        };
                    }
                }
                ProfileEditButton::Save => {
                    let Ok(avatar) = avatar_query.get_single() else {
                        return;
                    };
                    let Ok(profile_name) = changed_value.get_single() else {
                        return;
                    };
                    let mut profile = profile::get_profile(pkv.as_ref()).unwrap_or_default();
                    profile.avatar_id = avatar.id;
                    profile.name.clone_from(&profile_name.0);
                    profile.save_profile(&mut pkv);
                    next_state.set(GameState::Menu);
                }
            }
        }
    }
}

fn get_avatar_id(pkv: &PkvStore) -> i32 {
    match profile::get_profile(pkv) {
        Some(profile) => profile.avatar_id,
        None => {
            let mut rng = rand::thread_rng();
            rng.gen_range(1..consts::AVATARS_AMOUNT)
        }
    }
}

fn get_user_name(pkv: &PkvStore) -> String {
    match profile::get_profile(pkv) {
        Some(profile) => profile.name.clone(),
        None => "Wojmir".to_owned(),
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, pkv: Res<PkvStore>) {
    let ui_entity = commands
        .spawn(Node {
            height: FULL_SIZE_PERCENT,
            width: FULL_SIZE_PERCENT,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        // .insert(StyleSheet::new(asset_server.load("css/base.css")))
        .with_children(|parent| {
            parent.spawn((
                ImageNode::new(asset_server.load("img/start_screen_bg.png")),
                ZIndex(-1),
            ));
            // .insert(Class::new("menu_background"));

            parent
                .spawn((
                    ImageNode {
                        image_mode: bevy::ui::widget::NodeImageMode::Sliced(TextureSlicer {
                            border: BorderRect::square(29.0),
                            center_scale_mode: SliceScaleMode::Stretch,
                            sides_scale_mode: SliceScaleMode::Stretch,
                            max_corner_scale: 1.0,
                        }),
                        image: asset_server.load("img/panel-004.png"),
                        ..default()
                    },
                    BackgroundColor(Color::srgb_u8(110, 116, 77)),
                    Name::new("profile_edit"),
                    // Class::new("popup_window"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn(Node::default())
                        .with_child(Text::new("Edit profile"));

                    let init_scale = Vec3::splat(0.01);
                    let tween_scale = Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_millis(500),
                        TransformScaleLens {
                            start: init_scale,
                            end: Vec3::ONE,
                        },
                    );
                    parent.spawn((
                        BorderColor(Srgba::hex("fcfd9e").unwrap().into()),
                        BackgroundColor(Srgba::hex("2c422e").unwrap().into()),
                        Node {
                            width: Val::Px(200.0),
                            border: UiRect::all(Val::Px(5.0)),
                            padding: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        TextInput,
                        TextInputValue(get_user_name(pkv.as_ref())),
                        TextColor(Srgba::hex("fcfd9e").unwrap().into()),
                        TextInputSettings {
                            retain_on_submit: true,
                            ..Default::default()
                        }, // TextInputBundle::default()
                           //     .with_text_style(TextStyle {
                           //         font_size: 30.,
                           //         color: Srgba::hex("fcfd9e").unwrap().into(),
                           //         ..default()
                           //     })
                           //     .with_value(get_user_name(pkv.as_ref()))
                           //     .with_settings(TextInputSettings {
                           //         retain_on_submit: true,
                           //         ..default()
                           //     }),
                    ));

                    let avatar_id = get_avatar_id(pkv.as_ref());
                    let avatar_path = profile::get_avatar_path(avatar_id);
                    parent
                        .spawn((
                            ImageNode::new(asset_server.load(avatar_path)),
                            Name::new("avatar"),
                            Animator::new(tween_scale),
                            AvatarDisplay { id: avatar_id },
                        ))
                        .with_children(|av_root| {
                            for (text, class, label) in [
                                (">", "next", ProfileEditButton::NextAvatar),
                                ("<", "previous", ProfileEditButton::PreviousAvatar),
                            ] {
                                av_root
                                    .spawn((
                                        Button,
                                        BackgroundColor(Srgba::hex("7A444A").unwrap().into()),
                                        Transform::from_scale(init_scale),
                                        Node {
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        Animator::new(Tween::new(
                                            EaseFunction::QuadraticInOut,
                                            Duration::from_millis(500),
                                            TransformScaleLens {
                                                start: init_scale,
                                                end: Vec3::ONE,
                                            },
                                        )),
                                        // Class::new(format!("avatar common {}", class)),
                                        Name::new(format!("button_{}", class)),
                                        label,
                                    ))
                                    .with_child((
                                        Text::new(text),
                                        TextFont {
                                            font: asset_server.load(consts::LABEL_FONT),
                                            font_size: 30.0,
                                            ..default()
                                        },
                                        TextColor(Color::linear_rgb(0.7, 0.7, 0.7)),
                                    ));
                            }
                        });
                    let (text, label) = ("Save Profile", ProfileEditButton::Save);
                    parent
                        .spawn((
                            Button,
                            ImageNode::new(asset_server.load("img/panel-006.png")).with_mode(
                                bevy::ui::widget::NodeImageMode::Sliced(TextureSlicer {
                                    border: BorderRect::square(29.0),
                                    center_scale_mode: SliceScaleMode::Stretch,
                                    sides_scale_mode: SliceScaleMode::Stretch,
                                    max_corner_scale: 1.0,
                                }),
                            ),
                            Node {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            Transform::from_scale(init_scale),
                            BackgroundColor(Srgba::hex("7A444A").unwrap().into()),
                            // Class::new("menu common"),
                            Name::new(format!("button:{}", text)),
                            label,
                            Animator::new(Tween::new(
                                EaseFunction::QuadraticInOut,
                                Duration::from_millis(500),
                                TransformScaleLens {
                                    start: init_scale,
                                    end: Vec3::ONE,
                                },
                            )),
                        ))
                        .with_child((
                            Text::new(text),
                            TextFont {
                                font: asset_server.load(consts::LABEL_FONT),
                                font_size: 30.0,
                                ..default()
                            },
                            TextColor(Color::linear_rgb(0.7, 0.7, 0.7)),
                        ));
                });
        })
        .id();
    commands.insert_resource(ProfileSelectionData { ui_entity });
}

fn cleanup(mut commands: Commands, data: Res<ProfileSelectionData>) {
    commands.entity(data.ui_entity).despawn_recursive();
}
