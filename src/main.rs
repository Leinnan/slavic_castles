use std::env;
use std::path;

mod consts;
mod my_game;
mod player;
mod resource;
use crate::my_game::MyGame;

fn main() -> ggez::GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let window_mode = ggez::conf::WindowMode {
        width: 1280.0,
        height: 720.0,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };
    // Make a Context.
    let cb = ggez::ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(window_mode)
        .add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut cb.build()?;
    let my_game = &mut MyGame::new(ctx)?;
    ggez::event::run(ctx, event_loop, my_game)
}
