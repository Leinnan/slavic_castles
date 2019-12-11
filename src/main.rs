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
    run::<MyGame>(
        "Slavic castles!",
        Vector::new(consts::SCREEN_WIDTH, consts::SCREEN_HEIGHT),Settings {
            max_updates: 60,
            ..Settings::default()
        });
}
