use crate::consts;
use crate::resource::*;
use rand::prelude::*;
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
    shake_duration: f64,
    offset: (f32,f32),
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
            shake_duration: 0.0,
            offset: (0f32,0f32),
        };

        Ok(result)
    }

    pub fn update_values(&mut self, resource: &Resource) {
        if self.amount > resource.amount || self.production > resource.production {
            self.shake_duration = 0.4;
        }
        self.amount = resource.amount;
        self.production = resource.production;
    }

    pub fn update(&mut self, delta: f64) {
        if self.shake_duration >= 0.0 {
            self.shake_duration -= delta;
            let mut rng = thread_rng();
            self.offset = (rng.gen_range(-5.0, 5.0),rng.gen_range(-15.0, 15.0));
        } else {
            self.offset = (0.0,0.0);
        }
    }

    pub fn draw(&mut self, window: &mut Window, x: f32, y: f32) -> Result<()> {
        let base_pos = (x + self.offset.0, y + self.offset.1);
        let bg = Rectangle {
            pos: base_pos.into(),
            size: Vector { x: SIZE, y: SIZE },
        };
        window.draw(&bg, Col(self.color));
        let mut is_draw_ok;
        is_draw_ok = self.icon.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((base_pos.0 + (SIZE / 2.0), base_pos.1 + (SIZE / 2.0))),
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
                    .with_center((base_pos.0 + (SIZE / 2.0), base_pos.1 + (SIZE / 2.0))),
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
            window.draw(&text.area().with_center((base_pos.0 + 17.0, base_pos.1 + 17.0)), Img(&text));
            Ok(())
        });

        if !is_draw_ok.is_ok() {
            return is_draw_ok;
        }

        is_draw_ok = self.font.execute(|f| {
            let text = f.render(&prod_text, &style)?;
            window.draw(
                &text.area().with_center((base_pos.0 + 17.0, base_pos.1 + SIZE - 17.0)),
                Img(&text),
            );
            Ok(())
        });
        is_draw_ok
    }
}
