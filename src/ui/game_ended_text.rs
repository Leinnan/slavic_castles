use crate::consts;
use crate::ui::animations;
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Blended, Background::Col, Background::Img, Color, Font, FontStyle},
    lifecycle::{Asset, Window},
    sound::Sound,
    Result,
};

const BG_AREA: Rectangle = Rectangle {
    pos: Vector {
        x: 0.0,
        y: (consts::SCREEN_HEIGHT / 2.0) - 50.0,
    },
    size: Vector {
        x: consts::SCREEN_WIDTH,
        y: 100.0,
    },
};

pub struct GameEndedText {
    font: Asset<Font>,
    result_text: String,
    enabled: bool,
    positive_sound: Asset<Sound>,
    negative_sound: Asset<Sound>,
    color: Color,
    text_scale_anim: animations::AnimationFloat,
    bg_alpha_anim: animations::AnimationFloat,
}

impl GameEndedText {
    pub fn new() -> GameEndedText {
        GameEndedText {
            font: Asset::new(Font::load("coolvetica.ttf")),
            result_text: "".to_string(),
            enabled: false,
            positive_sound: Asset::new(Sound::load("snd_win.mp3")),
            negative_sound: Asset::new(Sound::load("snd_lose.wav")),
            color: Color::WHITE,
            text_scale_anim: animations::AnimationFloat::new(0.0, 45.0, 0.3, 0.4),
            bg_alpha_anim: animations::AnimationFloat::new(0.0, 0.7, 0.0, 0.45),
        }
    }

    pub fn enable(&mut self, enable: bool) {
        self.text_scale_anim.reset();
        self.bg_alpha_anim.reset();
        self.enabled = enable;
    }

    pub fn is_hovered(&self, pos: Vector) -> bool {
        self.enabled && BG_AREA.contains(pos)
    }

    pub fn game_ended(&mut self, positive: bool) {
        self.enable(true);
        self.result_text = if positive {
            "You win! Press to restart!".to_string()
        } else {
            "You lose! Press to restart!".to_string()
        };
        if positive {
            self.color = Color::WHITE;
            self.positive_sound.execute(|sound| {
                sound.play()?;
                Ok(())
            });
        } else {
            self.color = Color::RED;
            self.negative_sound.execute(|sound| {
                sound.play()?;
                Ok(())
            });
        }
    }

    pub fn update(&mut self, delta: f64) {
        if self.enabled {
            self.text_scale_anim.update(delta);
            self.bg_alpha_anim.update(delta);
        }
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        let bg_alpha = self.bg_alpha_anim.get_current_value();
        let center = Vector::new(consts::SCREEN_WIDTH / 2.0, consts::SCREEN_HEIGHT / 2.0);
        window.draw_ex(
            &BG_AREA,
            Col(Color::BLACK.with_alpha(bg_alpha)),
            Transform::IDENTITY,
            14,
        );

        let text = format!("{}", self.result_text);
        let color = self.color;
        let font_scale = self.text_scale_anim.get_current_value();

        self.font.execute(|f| {
            let style = FontStyle::new(font_scale, color);
            let text = f.render(&text, &style)?;
            window.draw_ex(
                &text.area().with_center(center),
                Img(&text),
                Transform::IDENTITY,
                15,
            );
            Ok(())
        })
    }
}
