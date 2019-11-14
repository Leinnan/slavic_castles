use crate::consts;
use nalgebra;
use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Col, Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};
use std::collections::VecDeque;

type Point2 = nalgebra::Point2<f32>;

const BG_AREA: Rectangle = Rectangle {
    pos: Vector { x: 300.0, y: 400.0 },
    size: Vector { x: 300.0, y: 300.0 },
};

pub struct Console {
    infos: VecDeque<String>,
    visible: bool,
    font: Asset<Font>,
}

impl Console {
    pub fn new() -> Result<Console> {
        Ok(Console {
            infos: VecDeque::with_capacity(10),
            visible: true,
            font: Asset::new(Font::load("coolvetica.ttf")),
        })
    }

    pub fn switch_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn message(&mut self, msg: &str) {
        if self.infos.len() == self.infos.capacity() {
            self.infos.pop_front();
        }
        self.infos.push_back(msg.to_string());
    }

    pub fn clear(&mut self) {
        self.infos = VecDeque::with_capacity(10);
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        if !self.visible {
            return Ok(());
        }
        let (w, h) = (1280, 720);
        let size_and_pos = Point2::new(w as f32 / 2.0 - 10.0, h as f32 / 2.0 - 10.0);
        // window.draw(&BG_AREA, Col(Color::BLUE));

        let mut result = String::from("Info:\n");
        for el in &self.infos {
            result.push_str(el.as_str());
            result.push_str("\n");
        }

        self.font.execute(|f| {
            let style = FontStyle::new(23.0, Color::BLACK);
            let text = f.render(&result, &style)?;
            window.draw(&text.area(), Img(&text));
            Ok(())
        })
    }
}
