use crate::card::Card;
use crate::consts;
use crate::resource::*;
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
    can_afford: bool,
    ready: bool,
    pos_x: f32,
    pos_y: f32,
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
            ResourceType::Magic => consts::MAGIC_COLOR.into(),
            ResourceType::Soldiers => consts::SOLDIERS_COLOR.into(),
            ResourceType::Tools => consts::TOOLS_COLOR.into(),
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

    pub fn draw(&self, ctx: &mut Context, font: graphics::Font) {
        if !self.ready {
            return;
        }
        let txt_color = if self.can_afford {
            consts::FONT_WHITE_COLOR.into()
        } else {
            consts::FONT_GREY_COLOR.into()
        };
        graphics::draw(
            ctx,
            &self.bg,
            graphics::DrawParam::default()
                .dest(Point2::new(self.pos_x, self.pos_y))
                .color(self.color),
        );
        graphics::draw(
            ctx,
            &self.front,
            graphics::DrawParam::default().dest(Point2::new(self.pos_x, self.pos_y)),
        );

        let cost_text = graphics::Text::new((format!("{}", self.cost), font, consts::TEXT_SIZE));
        graphics::draw(
            ctx,
            &cost_text,
            graphics::DrawParam::default()
                .dest(Point2::new(self.pos_x + 26.0, self.pos_y + 18.0))
                .color(txt_color)
                .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]),
        );

        let description_text =
            graphics::Text::new((format!("{}", self.description), font, consts::TEXT_SIZE));
        graphics::draw(
            ctx,
            &description_text,
            graphics::DrawParam::default()
                .dest(Point2::new(self.pos_x + 27.0, self.pos_y + 170.0))
                .color(txt_color)
                .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]),
        );
    }
}
