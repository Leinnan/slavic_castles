use crate::card::Card;
use crate::consts;
use crate::resource::*;
use crate::ui::animations;
use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{
        Background::Blended, Background::Col, Background::Img, Color, Font, FontStyle, Image,
    },
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};

pub struct CardDisplayer {
    bg: Asset<Image>,
    color: Color,
    front: Asset<Image>,
    font: Asset<Font>,
    cost: i32,
    description: String,
    can_afford: bool,
    hovered: bool,
    ready: bool,
    pos_x: f32,
    scale_anim: animations::AnimationFloat,
    pos_y_anim: animations::AnimationFloat,
}

impl CardDisplayer {
    pub fn new(base_scale: f32, x: f32, y: f32, y_offset: f32) -> Result<CardDisplayer> {
        let result = CardDisplayer {
            bg: Asset::new(Image::load("card_bg.png")),
            color: consts::SOLDIERS_COLOR,
            front: Asset::new(Image::load("card_front.png")),
            cost: 0,
            font: Asset::new(Font::load("coolvetica.ttf")),
            description: "".to_string(),
            can_afford: false,
            hovered: false,
            ready: false,
            pos_x: x,
            scale_anim: animations::AnimationFloat::new(0.9 * base_scale, base_scale, 0.0, 0.3),
            pos_y_anim: animations::AnimationFloat::new(y, y + y_offset, 0.0, 0.3),
        };
        Ok(result)
    }

    pub fn show(&mut self, show: bool) {
        self.pos_y_anim.play(show, false);
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.scale_anim.play(!hovered, false);
        self.hovered = hovered;
    }

    pub fn is_pos_over(&self, x: f32, y: f32) -> bool {
        let pos = self.get_center_pos();
        let scale = self.get_scale();
        let start_x = pos[0] + 10.0 - consts::CARD_SIZE_X / 2.0 * scale[0];
        let end_x = pos[0] - 10.0 + consts::CARD_SIZE_X / 2.0 * scale[0];
        let start_y = pos[1] + 10.0 - consts::CARD_SIZE_X / 2.0 * scale[1];
        let end_y = pos[1] - 10.0 + consts::CARD_SIZE_X / 2.0 * scale[1];

        x >= start_x && x <= end_x && y >= start_y && y <= end_y
    }

    pub fn update(&mut self, delta_time: f64) {
        self.scale_anim.update(delta_time);
        self.pos_y_anim.update(delta_time);
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
        for el in card.effects.iter() {
            self.description.push_str(&format!("{}\n", el));
        }

        self.ready = true;
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        if !self.ready {
            return Ok(());
        }
        let mut is_ok;
        let pos = self.get_center_pos();
        let scale = self.get_scale();
        let color = if self.can_afford {
            self.color
        } else {
            self.color.multiply(consts::GREY)
        };
        is_ok = self.bg.execute(|image| {
            window.draw_ex(
                &image.area().with_center((pos[0], pos[1])),
                Blended(&image, color),
                Transform::scale((scale[0], scale[1])),
                0,
            );
            Ok(())
        });
        if !is_ok.is_ok() {
            return is_ok;
        }
        is_ok = self.front.execute(|image| {
            window.draw_ex(
                &image.area().with_center((pos[0], pos[1])),
                Img(&image),
                Transform::scale((scale[0], scale[1])),
                1,
            );
            Ok(())
        });

        if !is_ok.is_ok() {
            return is_ok;
        }
        let cost_text = format!("{}", self.cost);
        is_ok = self.font.execute(|f| {
            let style = FontStyle::new(30.0, Color::WHITE);
            let text = f.render(&cost_text, &style)?;
            window.draw_ex(
                &text
                    .area()
                    .with_center((pos[0] - 85.0 * scale[0], pos[1] - 130.0 * scale[1])),
                Img(&text),
                Transform::scale((scale[0], scale[1])),
                2,
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
            window.draw_ex(
                &text.area().with_center((pos[0], pos[1] + 25.0 * scale[1])),
                Img(&text),
                Transform::scale((scale[0], scale[1])),
                2,
            );
            Ok(())
        });

        is_ok
    }

    fn get_center_pos(&self) -> [f32; 2] {
        [
            self.pos_x + (consts::CARD_SIZE_X / 2.0),
            self.pos_y_anim.get_current_value() + (consts::CARD_SIZE_Y / 2.0),
        ]
    }

    fn get_scale(&self) -> [f32; 2] {
        if !self.can_afford {
            [self.scale_anim.start_value, self.scale_anim.start_value]
        } else {
            [
                self.scale_anim.get_current_value(),
                self.scale_anim.get_current_value(),
            ]
        }
    }
}
