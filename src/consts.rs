pub const BASE_RESOURCE_AMOUNT: i32 = 5;
pub const BASE_RESOURCE_PRODUCTION: i32 = 1;
pub const BASE_TOWER_HP: i32 = 20;
pub const BASE_WALLS_HP: i32 = 15;
pub const MAX_TOWER_HP: i32 = 100;
pub const MAX_WALLS_HP: i32 = 50;
pub const CARDS_IN_DECK: i32 = 5;
pub const DELAY_BETWEEN_MOVES: f64 = 0.3;
// texts
pub const HELP: &str = "______________\nHelp\n 1-4 use card\n Use with Shift to discard card\n M- show console\n R- restart game\n H- show this info\n______________";
// UI
pub const FONT_COLOR: (f32, f32, f32, f32) = (29.0 / 255.0, 53.0 / 255.0, 87.0 / 255.0, 1.0);
pub const FONT_WHITE_COLOR: (f32, f32, f32, f32) = (0.95, 0.95, 0.95, 1.0);
pub const FONT_GREY_COLOR: (f32, f32, f32, f32) = (0.65, 0.65, 0.65, 1.0);
pub const ACTIVE_FONT_COLOR: (f32, f32, f32, f32) =
    (230.0 / 255.0, 57.0 / 255.0, 70.0 / 255.0, 1.0);
pub const BG_COLOR: (f32, f32, f32, f32) = (241.0 / 255.0, 250.0 / 255.0, 238.0 / 255.0, 1.0);

pub const TOOLS_COLOR: (f32, f32, f32, f32) = (140.0 / 255.0, 193.0 / 255.0, 1.0, 1.0);
pub const MAGIC_COLOR: (f32, f32, f32, f32) = (118.0 / 255.0, 206.0 / 255.0, 113.0 / 255.0, 1.0);
pub const SOLDIERS_COLOR: (f32, f32, f32, f32) = (206.0 / 255.0, 55.0 / 255.0, 75.0 / 255.0, 1.0);

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
#[cfg(target_arch = "wasm32")]
pub const TEXT_SCALE: f32 = 1.7;
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

pub const CARD_SIZE_X: f32 = 234.0;
pub const CARD_SIZE_Y: f32 = 320.0;