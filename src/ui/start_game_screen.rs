use crate::consts;
use crate::ui::animations;
use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{
        Background::Blended, Background::Col, Background::Img, Color, Font, FontStyle, Image,
    },
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};

pub struct StartGameScreen {
    bg: Image,
    logo: Image,
    logo_scale_anim: animations::AnimationFloat,
    font: Asset<Font>,
    pub visible: bool,
}

impl StartGameScreen {
    pub fn new() -> Self {
        let bg = Image::from_bytes(&consts::START_SCREEN_BG_IMG);
        let logo = Image::from_bytes(&consts::LOGO_IMG);
        StartGameScreen {
            bg: bg.unwrap(),
            logo: logo.unwrap(),
            logo_scale_anim: animations::AnimationFloat::new(0.0, 1.0, 1.3, 3.3),
            font: Asset::new(Font::load("coolvetica.ttf")),
            visible: true,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.logo_scale_anim.update(delta_time);
    }

    pub fn draw(&mut self, window: &mut Window) {
        if !self.visible {
            return;
        }
        let screen_center = ((consts::SCREEN_WIDTH / 2.0), (consts::SCREEN_HEIGHT / 2.0));

        window.draw_ex(
            &self.bg.area().with_center(screen_center),
            Img(&self.bg),
            Transform::IDENTITY,
            0,
        );
        let scale = (
            self.logo_scale_anim.get_current_value(),
            self.logo_scale_anim.get_current_value(),
        );
        window.draw_ex(
            &self.logo.area().with_center((screen_center.0, 120.0)),
            Img(&self.logo),
            Transform::scale(scale),
            1,
        );

        self.font.execute(|f| {
            let style = FontStyle::new(45.0, consts::FONT_WHITE_COLOR);
            let text = f.render("Press any key to start", &style)?;
            window.draw_ex(
                &text.area().with_center(screen_center),
                Img(&text),
                Transform::scale(scale),
                2,
            );
            Ok(())
        });
    }
}
