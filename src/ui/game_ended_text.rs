use crate::consts;
use ggez::{graphics, Context};

type Point2 = ggez::nalgebra::Point2<f32>;

pub struct GameEndedText {
    player_name: String,
}

impl GameEndedText {
    pub fn new() -> GameEndedText {
        GameEndedText {
            player_name: "".to_string(),
        }
    }

    pub fn set_player_name(&mut self, name: String) {
        self.player_name = name;
    }

    pub fn draw(&self, ctx: &mut Context, font: graphics::Font) {
        let (w, h) = graphics::drawable_size(ctx);
        let size = Point2::new(w as f32, 80.0);
        let pos = Point2::new(0.0, h as f32 / 2.0 - 26.0);

        let drawparams = graphics::DrawParam::new()
            .dest(pos)
            .color(consts::FONT_COLOR.into())
            .rotation(0.0 as f32)
            .offset(Point2::new(0.0, 0.0));

        let mut text =
            graphics::Text::new((format!("Game Ended, {} wins", self.player_name), font, 26.0));
        //text.set_bounds(size, graphics::Align::Center);

        graphics::draw(ctx, &text, drawparams);
    }
}
