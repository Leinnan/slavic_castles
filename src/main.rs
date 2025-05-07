use bevy::ecs::error::{GLOBAL_ERROR_HANDLER, warn};
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
// use bevy_ecss::prelude::*;
use bevy_pkv::PkvStore;
use data::deck::DeckAsset;
use states::game::NamesAsset;
pub mod base_systems;
pub mod components;
pub mod data;
pub mod helpers;
pub mod states;
pub mod visual;

const NAME: &str = env!("CARGO_PKG_NAME");

#[bevy_main]
pub fn main() {
    GLOBAL_ERROR_HANDLER
        .set(warn)
        .expect("The error handler can only be set once, globally.");
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();
    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: NAME.to_owned(),
            #[cfg(not(debug_assertions))]
            mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            #[cfg(debug_assertions)]
            mode: bevy::window::WindowMode::Windowed,
            ..default()
        }),
        ..default()
    }));
    #[cfg(target_arch = "wasm32")]
    app.add_plugins(DefaultPlugins);

    app.add_plugins(helpers::plugin)
        .add_plugins(JsonAssetPlugin::<DeckAsset>::new(&["deck.json"]))
        .add_plugins(JsonAssetPlugin::<NamesAsset>::new(&["names.json"]))
        .add_plugins(base_systems::turn_based::register_system)
        .add_plugins(components::player_ui::PlayerUiPlugin)
        .init_state::<states::game_states::GameState>()
        .add_plugins(helpers::wasm_resize::WindowResizePlugin)
        .register_type::<DeckAsset>()
        .register_type::<NamesAsset>()
        .add_plugins(visual::plugin)
        .add_plugins(game_core::GameCorePlugin)
        .insert_resource(PkvStore::new("CoolGames", NAME))
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
    if !input.just_released(KeyCode::F11) {}
    // let mut window = windows.single_mut();
    // let new_mode = match &window.mode {
    //     bevy::window::WindowMode::Windowed => bevy::window::WindowMode::Fullscreen,
    //     _ => bevy::window::WindowMode::Windowed,
    // };
    // window.mode = new_mode;
}
