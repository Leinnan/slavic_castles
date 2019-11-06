use crate::consts;
use crate::resource::*;
use crate::card::Card;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

type Point2 = na::Point2<f32>;

pub struct CardDisplayer {
    bg: graphics::Image,
    color: graphics::Color,
    front: graphics::Image,
    cost: i32,
    description: String,
    ready: bool
}

impl CardDisplayer {
    pub fn new(ctx: &mut Context) -> GameResult<CardDisplayer> {
        let bg = graphics::Image::new(ctx, "/card_bg.png")?;
        let front = graphics::Image::new(ctx, "/card_front.png")?;

        let result = CardDisplayer {
            bg: bg,
            color: consts::SOLDIERS_COLOR.into(),
            front: front,
            cost: 0,
            description: "".to_string(),
            ready: false,
        };
        Ok(result)
    }

    pub fn update_info(&mut self, card: &Card) {
        self.cost = card.cost_amount;
        self.color = match card.cost_resource {
            ResourceType::Magic => consts::MAGIC_COLOR.into(),
            ResourceType::Soldiers => consts::SOLDIERS_COLOR.into(),
            ResourceType::Tools => consts::TOOLS_COLOR.into(),
        };
        self.ready = true;
    }

    pub fn draw(&self, ctx: &mut Context, font: graphics::Font, x: f32, y: f32) {
        if !self.ready {
            return;
        }
        graphics::draw(
            ctx,
            &self.bg,
            graphics::DrawParam::default().dest(Point2::new(x,y)).color(self.color),
        );
        graphics::draw(
            ctx,
            &self.front,
            graphics::DrawParam::default().dest(Point2::new(x,y)),
        );

        let cost_text =
            graphics::Text::new((format!("{}", self.cost), font, consts::TEXT_SIZE));
        graphics::draw(
            ctx,
            &cost_text,
            graphics::DrawParam::default()
                .dest(Point2::new(x + 10.0, y + 28.0))
                .color((consts::FONT_COLOR).into())
                .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]));
    }
}