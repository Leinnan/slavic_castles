use crate::consts;
use ggez::{graphics, Context};
use std::collections::VecDeque;

type Point2 = ggez::nalgebra::Point2<f32>;

pub struct Console {
    infos: VecDeque<String>,
    visible: bool,
}

impl Console {
    pub fn new() -> Console {
        let infos = VecDeque::with_capacity(10);

        Console {
            infos,
            visible: true,
        }
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

    pub fn draw(&self, ctx: &mut Context, font: graphics::Font) {
        if !self.visible {
            return;
        }
        let size_and_pos = Point2::new(
            consts::SCREEN_WIDTH / 2.0 - 10.0,
            consts::SCREEN_HEIGHT / 2.0 - 10.0,
        );

        let drawparams = graphics::DrawParam::new()
            .dest(size_and_pos)
            .color(consts::FONT_COLOR.into())
            .rotation(0.0 as f32)
            .offset(Point2::new(0.0, 0.0));

        let mut result = String::from("Info:\n");
        for el in &self.infos {
            result.push_str(el.as_str());
            result.push_str("\n");
        }

        let mut text = graphics::Text::new((format!("{}", result), font, 18.0));
        text.set_bounds(size_and_pos, graphics::Align::Right);

        graphics::draw(ctx, &text, drawparams);
    }
}
