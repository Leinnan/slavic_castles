use crate::consts;
use crate::ui::animations;
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{
        Background::Blended, Background::Col, Background::Img, Color, Font, FontStyle, Image,
    },
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};

const BG_AREA: Rectangle = Rectangle {
    pos: Vector { x: 0.0, y: 0.0 },
    size: Vector { x: 320.0, y: 210.0 },
};

pub struct HelpDisplayer {
    font: Asset<Font>,
    visible: bool,
}

impl HelpDisplayer {
    pub fn new() -> Result<Self> {
        Ok(HelpDisplayer {
            font: Asset::new(Font::load("coolvetica.ttf")),
            visible: true,
        })
    }

    pub fn switch_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        if !self.visible {
            return Ok(());
        }
        let center = Vector::new(
            consts::SCREEN_WIDTH / 2.0,
            consts::SCREEN_HEIGHT / 2.0 - 120.0,
        );
        window.draw_ex(
            &BG_AREA.with_center(center),
            Col(Color::BLACK.with_alpha(0.4)),
            Transform::IDENTITY,
            10,
        );
        self.font.execute(|f| {
            let style = FontStyle::new(20.0, Color::WHITE);
            let text = f.render(consts::HELP_TEXT, &style)?;
            window.draw_ex(
                &text.area().with_center(center),
                Img(&text),
                Transform::IDENTITY,
                11,
            );
            Ok(())
        });
        return Ok(());
    }
}
