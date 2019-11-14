extern crate quicksilver;

mod card;
mod consts;
mod deck;
mod my_game;
mod player;
mod resource;
mod ui;

use crate::my_game::MyGame;
use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings},
};

fn main() {
    run::<MyGame>(
        "Slavic castles!",
        Vector::new(consts::SCREEN_WIDTH, consts::SCREEN_HEIGHT),
        Settings::default(),
    );
}
