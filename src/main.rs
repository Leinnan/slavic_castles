extern crate quicksilver;

mod board;
mod card;
mod card_effect;
mod card_sounds;
mod consts;
mod deck;
mod my_game;
mod player;
mod resource;
mod stats;
mod ui;

use crate::my_game::MyGame;
use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings},
};

fn main() {
    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");
    run::<MyGame>(
        "Slavic castles!",
        Vector::new(consts::SCREEN_WIDTH, consts::SCREEN_HEIGHT),Settings {
            scale: quicksilver::graphics::ImageScaleStrategy::Blur,
            resize: quicksilver::graphics::ResizeStrategy::Maintain,
            draw_rate:  16.0,  // 16ms per draw = 60 draw per second
            max_updates: 60, // Maximum updates per frame
            ..Settings::default()
        });
}
