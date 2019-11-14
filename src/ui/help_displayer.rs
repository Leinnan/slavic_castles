use crate::consts;
use quicksilver::{
    Future, Result,
    combinators::result,
    geom::{Shape, Rectangle, Vector},
    graphics::{Background::Img, Background::Col, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Settings, State, Window, run},
};
use nalgebra;

type Point2 = nalgebra::Point2<f32>;
// use ggez::{graphics, Context, GameResult};
// use std::collections::VecDeque;

// type Point2 = ggez::nalgebra::Point2<f32>;

pub struct HelpDisplayer {
    // empty_bg: graphics::Image,
    visible: bool,
}

impl HelpDisplayer {
    pub fn new() -> Result<Self> {
        // let empty_bg = graphics::Image::new(ctx, "/empty.png")?;

        let result = HelpDisplayer {
            // empty_bg,
            visible: true,
        };
        Ok(result)
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
        // let (w, h) = graphics::drawable_size(ctx);
        // let size_and_pos = Point2::new(400.0, 30.0);
        // graphics::draw(
        //     ctx,
        //     &self.empty_bg,
        //     graphics::DrawParam::default()
        //         .dest(size_and_pos)
        //         .scale([330.0, 230.0])
        //         .color((0.0, 0.0, 0.0, 0.6).into()),
        // );

        // let drawparams = graphics::DrawParam::default()
        //     .dest(Point2::new(410.0, 40.0))
        //     .color(consts::FONT_WHITE_COLOR.into())
        //     .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]);

        // let text = graphics::Text::new((
        //     format!("{}", consts::HELP_TEXT),
        //     font,
        //     consts::TEXT_SIZE * 1.2,
        // ));

        // graphics::draw(ctx, &text, drawparams);
        return Ok(());
    }
}
