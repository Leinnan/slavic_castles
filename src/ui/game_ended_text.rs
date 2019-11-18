use crate::consts;
use quicksilver::{
    geom::{Shape, Vector},
    graphics::{Background::Img, Color, Font, FontStyle},
    lifecycle::{Asset, Window},
    Result,
};

pub struct GameEndedText {
    font: Asset<Font>,
    player_name: String,
    enabled: bool,
}

impl GameEndedText {
    pub fn new() -> GameEndedText {
        GameEndedText {
            font: Asset::new(Font::load("coolvetica.ttf")),
            player_name: "".to_string(),
            enabled: false,
        }
    }

    pub fn enable(&mut self, enable: bool) {
        self.enabled = enable;
    }

    pub fn set_player_name(&mut self, name: String) {
        self.player_name = name;
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        let center = Vector::new(consts::SCREEN_WIDTH / 2.0, consts::SCREEN_HEIGHT / 2.0);

        let text = format!("Game Ended, {} wins, press R to restart", self.player_name);

        self.font.execute(|f| {
            let style = FontStyle::new(50.0, Color::WHITE);
            let text = f.render(&text, &style)?;
            window.draw(&text.area().with_center(center), Img(&text));
            Ok(())
        })
    }
}
