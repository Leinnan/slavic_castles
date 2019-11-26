use crate::consts;
use quicksilver::{
    geom::{Shape, Vector},
    graphics::{Background::Img, Color, Font, FontStyle},
    lifecycle::{Asset, Window},
    sound::Sound,
    Result,
};

pub struct GameEndedText {
    font: Asset<Font>,
    result_text: String,
    enabled: bool,
    positive_sound: Asset<Sound>,
    negative_sound: Asset<Sound>,
}

impl GameEndedText {
    pub fn new() -> GameEndedText {
        GameEndedText {
            font: Asset::new(Font::load("coolvetica.ttf")),
            result_text: "".to_string(),
            enabled: false,
            positive_sound: Asset::new(Sound::load("snd_win.mp3")),
            negative_sound: Asset::new(Sound::load("snd_lose.wav")),
        }
    }

    pub fn enable(&mut self, enable: bool) {
        self.enabled = enable;
    }

    pub fn game_ended(&mut self, positive: bool) {
        self.enable(true);
        self.result_text = if positive {
            "You win! Press R to restart!".to_string()
        } else {
            "You lose! Press R to restart!".to_string()
        };
        if positive {
            self.positive_sound.execute(|sound| {
                sound.play()?;
                Ok(())
            });
        } else {
            self.negative_sound.execute(|sound| {
                sound.play()?;
                Ok(())
            });
        }
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        let center = Vector::new(consts::SCREEN_WIDTH / 2.0, consts::SCREEN_HEIGHT / 2.0);

        let text = format!("{}", self.result_text);

        self.font.execute(|f| {
            let style = FontStyle::new(50.0, Color::WHITE);
            let text = f.render(&text, &style)?;
            window.draw(&text.area().with_center(center), Img(&text));
            Ok(())
        })
    }
}
