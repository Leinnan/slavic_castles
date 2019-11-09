use crate::consts;
use ggez::{graphics, Context, GameResult};
use std::collections::VecDeque;

type Point2 = ggez::nalgebra::Point2<f32>;

pub struct Console {
    empty_bg: graphics::Image,
    infos: VecDeque<String>,
    visible: bool,
}

impl Console {
    pub fn new(ctx: &mut Context) -> GameResult<Console> {
        let infos = VecDeque::with_capacity(10);
        let empty_bg = graphics::Image::new(ctx, "/empty.png")?;

        let result = Console {
            empty_bg,
            infos,
            visible: false,
        };
        Ok(result)
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
        let (w, h) = graphics::drawable_size(ctx);
        let size_and_pos = Point2::new(w as f32 / 2.0 - 10.0, h as f32 / 2.0 - 10.0);
        graphics::draw(
            ctx,
            &self.empty_bg,
            graphics::DrawParam::default()
                .dest(size_and_pos)
                .scale([w as f32 / 2.0 - 10.0, h as f32 / 2.0 - 10.0])
                .color((0.0, 0.0, 0.0, 0.4).into()),
        );

        let drawparams = graphics::DrawParam::default()
            .dest(Point2::new(w as f32 / 2.0, h as f32 / 2.0))
            .color(consts::FONT_WHITE_COLOR.into())
            .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]);

        let mut result = String::from("Info:\n");
        for el in &self.infos {
            result.push_str(el.as_str());
            result.push_str("\n");
        }

        let text = graphics::Text::new((format!("{}", result), font, consts::TEXT_SIZE * 0.8));

        graphics::draw(ctx, &text, drawparams);
    }
}
