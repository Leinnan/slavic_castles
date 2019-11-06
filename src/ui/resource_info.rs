use crate::consts;
use crate::resource::*;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

type Point2 = na::Point2<f32>;

pub const SIZE: f32 = 90.0;

pub struct ResourceInfo {
    empty_bg: graphics::Image,
    icon: graphics::Image,
    frame: graphics::Image,
    amount: i32,
    production: i32,
    color: graphics::Color,
}

impl ResourceInfo {
    pub fn new(
        icon_path: String,
        color: graphics::Color,
        ctx: &mut Context,
    ) -> GameResult<ResourceInfo> {
        let icon = graphics::Image::new(ctx, icon_path)?;
        let frame = graphics::Image::new(ctx, "/resource_frame.png")?;
        let empty_bg = graphics::Image::new(ctx, "/empty.png")?;

        let result = ResourceInfo {
            empty_bg: empty_bg,
            icon: icon,
            frame: frame,
            amount: consts::BASE_RESOURCE_AMOUNT,
            production: consts::BASE_RESOURCE_PRODUCTION,
            color: color,
        };

        Ok(result)
    }

    pub fn update_values(&mut self, resource: &Resource) {
        self.amount = resource.amount;
        self.production = resource.production;
    }

    pub fn draw(&self, ctx: &mut Context, font: graphics::Font, x: f32, y: f32) -> GameResult<()> {
        graphics::draw(
            ctx,
            &self.empty_bg,
            graphics::DrawParam::default()
                .dest(Point2::new(x, y))
                .scale([SIZE, SIZE])
                .color(self.color),
        );

        graphics::draw(
            ctx,
            &self.icon,
            graphics::DrawParam::default().dest(Point2::new(x + 13.0, y + 13.0)),
        );
        graphics::draw(
            ctx,
            &self.frame,
            graphics::DrawParam::default().dest(Point2::new(x, y)),
        );

        let amount_text =
            graphics::Text::new((format!("{}", self.amount), font, consts::TEXT_SIZE));

        graphics::draw(
            ctx,
            &amount_text,
            graphics::DrawParam::default()
                .dest(Point2::new(x + 10.0, 28.0))
                .color((consts::FONT_COLOR).into())
                .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]),
        );
        let prod_text =
            graphics::Text::new((format!("+{}", self.production), font, consts::TEXT_SIZE));

        graphics::draw(
            ctx,
            &prod_text,
            graphics::DrawParam::default()
                .dest(Point2::new(x + 10.0, SIZE - 3.0))
                .color((consts::FONT_COLOR).into())
                .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]),
        )
    }
}
