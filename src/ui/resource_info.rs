use crate::consts;
use crate::resource::*;
use nalgebra;
use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Col, Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};

type Point2 = nalgebra::Point2<f32>;

pub const SIZE: f32 = 90.0;

pub struct ResourceInfo {
    icon: Asset<Image>,
    frame: Asset<Image>,
    font: Asset<Font>,
    amount: i32,
    production: i32,
    color: Color,
}

impl ResourceInfo {
    pub fn new(icon_path: String, color: Color) -> Result<ResourceInfo> {
        let result = ResourceInfo {
            icon: Asset::new(Image::load(icon_path)),
            frame: Asset::new(Image::load("resource_frame.png")),
            font: Asset::new(Font::load("coolvetica.ttf")),
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

    pub fn draw(&mut self, window: &mut Window, x: f32, y: f32) -> Result<()> {
        let bg = Rectangle {
            pos: Vector { x: x, y: y },
            size: Vector { x: SIZE, y: SIZE },
        };
        window.draw(&bg, Col(self.color));
        let mut is_draw_ok;
        is_draw_ok = self.icon.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((x + (SIZE / 2.0), y + (SIZE / 2.0))),
                Img(&image),
            );
            Ok(())
        });

        if !is_draw_ok.is_ok() {
            return is_draw_ok;
        }

        is_draw_ok = self.frame.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((x + (SIZE / 2.0), y + (SIZE / 2.0))),
                Img(&image),
            );
            Ok(())
        });

        if !is_draw_ok.is_ok() {
            return is_draw_ok;
        }

        let style = FontStyle::new(25.0, consts::FONT_COLOR);
        let amount_text = format!("{}", self.amount);
        let prod_text = format!("+{}", self.production);

        is_draw_ok = self.font.execute(|f| {
            let text = f.render(&amount_text, &style)?;
            window.draw(&text.area().with_center((x + 17.0, y + 17.0)), Img(&text));
            Ok(())
        });

        if !is_draw_ok.is_ok() {
            return is_draw_ok;
        }

        is_draw_ok = self.font.execute(|f| {
            let text = f.render(&prod_text, &style)?;
            window.draw(
                &text.area().with_center((x + 17.0, y + SIZE - 17.0)),
                Img(&text),
            );
            Ok(())
        });
        is_draw_ok
    }
}
