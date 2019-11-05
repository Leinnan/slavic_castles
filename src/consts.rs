pub const BASE_RESOURCE_AMOUNT: i32 = 5;
pub const BASE_RESOURCE_PRODUCTION: i32 = 1;
pub const BASE_TOWER_HP: i32 = 20;
pub const BASE_WALLS_HP: i32 = 15;
pub const MAX_TOWER_HP: i32 = 100;
pub const MAX_WALLS_HP: i32 = 50;
pub const CARDS_IN_DECK: i32 = 4;
pub const DELAY_BETWEEN_MOVES: f64 = 0.5;
// texts
pub const HELP: &str = "______________\nHelp\n 1-4 use card\n Use with Shift to discard card\n M- show console\n H- show this info\n______________";
// UI
pub const FONT_COLOR: (f32, f32, f32, f32) = (29.0 / 255.0, 53.0 / 255.0, 87.0 / 255.0, 1.0);
pub const ACTIVE_FONT_COLOR: (f32, f32, f32, f32) =
    (230.0 / 255.0, 57.0 / 255.0, 70.0 / 255.0, 1.0);
pub const BG_COLOR: (f32, f32, f32, f32) = (241.0 / 255.0, 250.0 / 255.0, 238.0 / 255.0, 1.0);
pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
#[cfg(target_arch = "wasm32")]
pub const TEXT_SCALE: f32 = 2.0;
#[cfg(not(target_arch = "wasm32"))]
pub const TEXT_SCALE: f32 = 1.0;
#[cfg(target_arch = "wasm32")]
pub const TEXT_SIZE: f32 = 13.0;
#[cfg(not(target_arch = "wasm32"))]
pub const TEXT_SIZE: f32 = 26.0;
#[cfg(target_arch = "wasm32")]
pub const FONT_WIDTH: f32 = 4.4;
#[cfg(not(target_arch = "wasm32"))]
pub const FONT_WIDTH: f32 = 5.3;
