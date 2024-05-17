use super::{game::NamesAsset, game_states::GameState};
use crate::{data::deck::DeckAsset, states};
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::*;
use consts::{FULL_SIZE_PERCENT, LABEL_FONT};
use states::consts;

#[allow(dead_code)]
#[derive(AssetCollection, Resource)]
pub struct BaseAssets {
    #[asset(
        paths(
            "css/base.css",
            "avatars/1.png",
            "avatars/2.png",
            "avatars/3.png",
            "avatars/4.png",
            "avatars/5.png",
            "avatars/6.png",
            "avatars/7.png",
            "avatars/8.png",
            "avatars/9.png",
            "avatars/10.png",
            "avatars/11.png",
            "avatars/12.png",
            "avatars/13.png",
            "cards/1.png",
            "cards/2.png",
            "cards/3.png",
            "cards/4.png",
            "cards/5.png",
            "cards/6.png",
            "cards/7.png",
            "cards/8.png",
            "cards/9.png",
            "cards/10.png",
            "cards/11.png",
            "cards/12.png",
            "cards/13.png",
            "cards/14.png",
            "cards/15.png",
            "cards/16.png",
            "cards/17.png",
            "cards/18.png",
            "cards/19.png",
            "cards/20.png",
            "cards/21.png",
            "cards/22.png",
            "cards/23.png",
            "cards/24.png",
            "cards/25.png",
            "cards/26.png",
            "img/panel-004.png",
            "img/panel-006.png",
            "img/start_screen_bg.png",
            "img/ingame_bg.png",
            "img/logo.png",
            "fonts/PirataOne-Regular.ttf",
            "fonts/AlteHaasGroteskBold.ttf",
            "snd/card_magic.ogg",
            "snd/card_soldiers.ogg",
            "snd/card_tools.ogg",
            "snd/card_dismiss.ogg",
        ),
        collection
    )]
    untyped_assets: Vec<UntypedHandle>,
    #[asset(path = "all.deck.json")]
    pub deck: Handle<DeckAsset>,
    #[asset(path = "enemy.names.json")]
    pub names: Handle<NamesAsset>,
}

pub struct LoadingPlugin;

#[derive(Component)]
pub struct LoadingText;

#[derive(Component)]
pub struct LoadingScreen;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetsLoading), cleanup)
            .add_loading_state(
                LoadingState::new(GameState::AssetsLoading)
                    .continue_to_state(GameState::Menu)
                    .load_collection::<BaseAssets>(),
            )
            .add_systems(OnEnter(GameState::AssetsLoading), setup_ui);
        //            .add_system(update_progress.in_set(OnUpdate(GameState::AssetsLoading)));
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: FULL_SIZE_PERCENT,
                    width: FULL_SIZE_PERCENT,
                    align_items: AlignItems::Stretch,
                    justify_content: JustifyContent::SpaceBetween,
                    align_content: AlignContent::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                background_color: Color::hex("#2c422e").unwrap().into(),
                ..default()
            },
            LoadingScreen,
        ))
        .with_children(|parent| {
            let header_style = TextStyle {
                font: asset_server.load(LABEL_FONT),
                font_size: 45.0,
                color: Color::hex("#fcfd9e").unwrap(),
            };
            parent.spawn((
                TextBundle::from_section("Loading", header_style).with_style(Style {
                    margin: UiRect::all(Val::Auto),
                    ..default()
                }),
                LoadingText,
            ));
        });
}

fn cleanup(query: Query<Entity, With<LoadingScreen>>, mut commands: Commands) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}
