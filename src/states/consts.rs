use bevy::prelude::*;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_HASH: &str = env!("GIT_HASH");
pub const NORMAL_BUTTON: Color = Color::rgb(1., 1., 1.);
pub const HOVERED_BUTTON: Color = Color::rgb(0.9, 0.9, 0.9);
pub const PRESSED_BUTTON: Color = Color::rgb(0.5, 0.5, 0.5);
pub const FULL_SIZE_PERCENT: Val = Val::Percent(100.);

pub const AVATARS_AMOUNT: i32 = 13;

pub const HELP_TEXT: &str = "Get 50 life or destroy opponent to win\n Drag card to center to use it\n or drag it down to discard.\n H- show this info";
pub const LABEL_FONT: &str = "fonts/PirataOne-Regular.ttf";
pub const REGULAR_FONT: &str = "fonts/AlteHaasGroteskBold.ttf";
