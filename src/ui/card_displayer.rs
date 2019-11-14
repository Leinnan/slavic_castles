use crate::card::Card;
use crate::consts;
use crate::resource::*;
use nalgebra;
use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Vector},
    graphics::{
        Background::Blended, Background::Col, Background::Img, Color, Font, FontStyle, Image,
    },
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};

type Point2 = nalgebra::Point2<f32>;

pub struct CardDisplayer {
    bg: Asset<Image>,
    color: Color,
    front: Asset<Image>,
    font: Asset<Font>,
    cost: i32,
    description: String,
    can_afford: bool,
    ready: bool,
    pos_x: f32,
    pos_y: f32,
}

impl CardDisplayer {
    pub fn new() -> Result<CardDisplayer> {
        let result = CardDisplayer {
            bg: Asset::new(Image::load("card_bg.png")),
            color: consts::SOLDIERS_COLOR,
            front: Asset::new(Image::load("card_front.png")),
            cost: 0,
            font: Asset::new(Font::load("coolvetica.ttf")),
            description: "".to_string(),
            can_afford: false,
            ready: false,
            pos_x: 0.0,
            pos_y: 0.0,
        };
        Ok(result)
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.pos_x = x;
        self.pos_y = y;
    }

    pub fn is_pos_over(&self, x: f32, y: f32) -> bool {
        let start_x = self.pos_x + 10.0;
        let end_x = self.pos_x + consts::CARD_SIZE_X - 10.0 * 2.0;
        let start_y = self.pos_y + 10.0;
        let end_y = self.pos_y + consts::CARD_SIZE_Y - 10.0 * 2.0;

        x >= start_x && x <= end_x && y >= start_y && y <= end_y
    }

    pub fn update_info(&mut self, card: &Card, can_afford: bool) {
        self.cost = card.cost_amount;
        self.can_afford = can_afford;
        self.color = match card.cost_resource {
            ResourceType::Magic => consts::MAGIC_COLOR,
            ResourceType::Soldiers => consts::SOLDIERS_COLOR,
            ResourceType::Tools => consts::TOOLS_COLOR,
        };

        self.description.clear();
        let growth = (card.tower_growth > 0, card.walls_growth > 0);
        match growth {
            (true, true) => self.description.push_str(&format!(
                "Adds {0} HP and {1} shield\n",
                card.tower_growth, card.walls_growth,
            )),
            (true, false) => self
                .description
                .push_str(&format!("Adds {0} HP\n", card.tower_growth,)),
            (false, true) => self
                .description
                .push_str(&format!("Adds {0} shield\n", card.walls_growth,)),
            _ => self.description.clear(),
        };
        if card.damage > 0 {
            self.description
                .push_str(&format!("{0} Damage to enemy\n", card.damage));
        }
        if card.production_change != 0 {
            self.description.push_str(&format!(
                "+{1} to {0} production",
                Resource::resource_name(&card.production_resource),
                card.production_change,
            ));
        }

        self.ready = true;
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        if !self.ready {
            return Ok(());
        }
        let mut is_ok;
        let pos = [
            self.pos_x + (consts::CARD_SIZE_X / 2.0),
            self.pos_y + (consts::CARD_SIZE_Y / 2.0),
        ];
        let color = if self.can_afford {
            self.color
        } else {
            self.color.multiply(consts::GREY)
        };
        is_ok = self.bg.execute(|image| {
            window.draw(
                &image.area().with_center((pos[0], pos[1])),
                Blended(&image, color),
            );
            Ok(())
        });
        if !is_ok.is_ok() {
            return is_ok;
        }
        is_ok = self.front.execute(|image| {
            window.draw(&image.area().with_center((pos[0], pos[1])), Img(&image));
            Ok(())
        });

        if !is_ok.is_ok() {
            return is_ok;
        }
        let cost_text = format!("{}", self.cost);
        is_ok = self.font.execute(|f| {
            let style = FontStyle::new(30.0, Color::WHITE);
            let text = f.render(&cost_text, &style)?;
            window.draw(
                &text.area().with_center((pos[0] - 85.0, pos[1] - 130.0)),
                Img(&text),
            );
            Ok(())
        });
        let result = self.description.clone();

        if !is_ok.is_ok() {
            return is_ok;
        }
        is_ok = self.font.execute(|f| {
            let style = FontStyle::new(23.0, Color::WHITE);
            let text = f.render(&result, &style)?;
            window.draw(
                &text.area().with_center((pos[0], pos[1] + 30.0)),
                Img(&text),
            );
            Ok(())
        });

        is_ok
    }
}
