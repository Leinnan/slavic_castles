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
        let pos = Point2::new(w as f32 - 150.0, h as f32 / 2.0 - 26.0);

        let drawparams = graphics::DrawParam::default()
            .dest(pos)
            .color(consts::FONT_COLOR.into())
            .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]);

        let text =
            graphics::Text::new((format!("Game Ended, {} wins, press R to restart", self.player_name), font, consts::TEXT_SIZE));

        graphics::draw(ctx, &text, drawparams);
    }
}
