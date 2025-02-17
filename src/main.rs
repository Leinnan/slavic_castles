use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
// use bevy_ecss::prelude::*;
use bevy_pkv::PkvStore;
use data::{deck::DeckAsset, player::Player, player_resources::PlayerResources};
use states::game::NamesAsset;
pub mod base_systems;
pub mod components;
pub mod data;
pub mod helpers;
pub mod states;

const NAME: &str = env!("CARGO_PKG_NAME");

#[bevy_main]
pub fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();
    // app.insert_resource(AssetMetaCheck::Never);
    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: NAME.to_owned(),
            #[cfg(not(debug_assertions))]
            mode: bevy::window::WindowMode::Fullscreen,
            #[cfg(debug_assertions)]
            mode: bevy::window::WindowMode::Windowed,
            ..default()
        }),
        ..default()
    }));
    #[cfg(target_arch = "wasm32")]
    app.add_plugins(DefaultPlugins);

    app.add_plugins(bevy_button_released_plugin::ButtonsReleasedPlugin)
        .add_plugins(JsonAssetPlugin::<DeckAsset>::new(&["deck.json"]))
        .add_plugins(JsonAssetPlugin::<NamesAsset>::new(&["names.json"]))
        .add_plugins(crate::base_systems::turn_based::register_system)
        .add_plugins(components::player_ui::PlayerUiPlugin)
        .init_state::<states::game_states::GameState>()
        // .add_plugins(bevy_wasm_window_resize::WindowResizePlugin)
        .register_type::<PlayerResources>()
        .register_type::<DeckAsset>()
        .register_type::<NamesAsset>()
        .register_type::<Player>()
        .insert_resource(PkvStore::new("MevLysdhkin", NAME))
        // .add_plugins(EcssPlugin::default())
        .add_plugins(bevy_tweening::TweeningPlugin)
        .add_plugins(states::game_states::GamePlugins)
        .add_systems(Update, toggle_fullscreen);

    #[cfg(debug_assertions)]
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_plugins(base_systems::debug::DebugPlugin);
    }

    app.run();
}

fn toggle_fullscreen(mut _windows: Query<&mut Window>, input: Res<ButtonInput<KeyCode>>) {
    if !input.just_released(KeyCode::F11) {
    }
    // let mut window = windows.single_mut();
    // let new_mode = match &window.mode {
    //     bevy::window::WindowMode::Windowed => bevy::window::WindowMode::Fullscreen,
    //     _ => bevy::window::WindowMode::Windowed,
    // };
    // window.mode = new_mode;
}
