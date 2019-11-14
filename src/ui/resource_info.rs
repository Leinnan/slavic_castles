use crate::consts;
use crate::resource::*;
use quicksilver::{
    Future, Result,
    combinators::result,
    geom::{Shape, Rectangle, Vector},
    graphics::{Background::Img, Background::Col, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Settings, State, Window, run},
};
use nalgebra;

type Point2 = nalgebra::Point2<f32>;

pub const SIZE: f32 = 90.0;

pub struct ResourceInfo {
    icon: Asset<Image>,
    // frame: graphics::Image,
    amount: i32,
    production: i32,
    color: Color,
}

impl ResourceInfo {
    pub fn new(
        icon_path: String,
        color: Color,
    ) -> Result<ResourceInfo> {

        let result = ResourceInfo {
            // empty_bg: empty_bg,
            icon: Asset::new(Image::load(icon_path)),
            // frame: frame,
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
            pos:  Vector {x: x, y: y},
            size: Vector {x: SIZE, y: SIZE}
        };
        window.draw(&bg, Col(self.color));

        self.icon.execute(|image| {
            window.draw(&image.area().with_center((x+13.0, y+13.0)), Img(&image));
            Ok(())
        })
        // graphics::draw(
        //     ctx,
        //     &self.frame,
        //     graphics::DrawParam::default().dest(Point2::new(x, y)),
        // );

        // let amount_text =
        //     graphics::Text::new((format!("{}", self.amount), font, consts::TEXT_SIZE));

        // graphics::draw(
        //     ctx,
        //     &amount_text,
        //     graphics::DrawParam::default()
        //         .dest(Point2::new(x + 10.0, 28.0))
        //         .color((consts::FONT_COLOR).into())
        //         .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]),
        // );
        // let prod_text =
        //     graphics::Text::new((format!("+{}", self.production), font, consts::TEXT_SIZE));

        // graphics::draw(
        //     ctx,
        //     &prod_text,
        //     graphics::DrawParam::default()
        //         .dest(Point2::new(x + 10.0, SIZE - 3.0))
        //         .color((consts::FONT_COLOR).into())
        //         .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]),
        // )
    }
}
