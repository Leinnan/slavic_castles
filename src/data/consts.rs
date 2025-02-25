pub const CARDS_IN_DECK: i32 = 5;
pub const BASE_RESOURCE_AMOUNT: i32 = 3;
pub const BASE_RESOURCE_PRODUCTION: i32 = 1;
pub const BASE_TOWER_HP: i32 = 15;
pub const BASE_WALLS_HP: i32 = 5;
pub const MAX_TOWER_HP: i32 = 50;
pub const MAX_WALLS_HP: i32 = 30;
pub const MAX_PRODUCTION: i32 = 10;

// pub const MY_ACCENT_COLOR: Color = Color::RgbaLinear {
//     red: 0.901,
//     green: 0.4,
//     blue: 0.01,
//     alpha: 1.0,
// };

#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
pub const MY_ACCENT_COLOR32: bevy_inspector_egui::bevy_egui::egui::Color32 =
    bevy_inspector_egui::bevy_egui::egui::Color32::from_rgb(230, 102, 1);
pub const GIT_HASH: &str = env!("GIT_HASH");
pub const GIT_DATE: &str = env!("GIT_DATE");
