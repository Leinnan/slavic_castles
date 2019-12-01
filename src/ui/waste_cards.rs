use crate::card::Card;
use crate::consts;
use crate::resource::*;
use crate::ui::animations;
use crate::ui::card_displayer::CardDisplayer;
use nalgebra;
use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{
        Background::Blended, Background::Col, Background::Img, Color, Font, FontStyle, Image,
    },
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};

pub struct WasteCards {
    card_back: Asset<Image>,
    previous_card: CardDisplayer,
    last_card: CardDisplayer,
    pub display_card: bool,
    cards_played: i32,
    x: f32,
    y: f32,
    scale: f32,
}

impl WasteCards {
    pub fn new(base_scale: f32, x: f32, y: f32) -> Result<Self> {
        let card = CardDisplayer::new(
            base_scale / 0.9,
            x + (base_scale * consts::CARD_SIZE_X),
            y,
            0.0,
        )?;
        let second_card = CardDisplayer::new(
            base_scale / 0.9,
            x + (base_scale * consts::CARD_SIZE_X * 2.0),
            y,
            0.0,
        )?;
        Ok(WasteCards {
            card_back: Asset::new(Image::load("card_back.png")),
            previous_card: card,
            last_card: second_card,
            x: x,
            y: y,
            display_card: true,
            cards_played: 0,
            scale: base_scale,
        })
    }

    pub fn card_used(&mut self, card: &Card) {
        self.cards_played += 1;
        if self.cards_played == 1 {
            self.previous_card.update_info(card, false);
        } else if self.cards_played == 2 {
            self.last_card.update_info(card, false);
        } else {
            self.previous_card.copy_info(&self.last_card);
            self.last_card.update_info(card, false);
        }
    }

    pub fn game_ended(&mut self) {
        self.cards_played = 0;
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let mut is_ok;
        let pos = [
            self.x + (consts::CARD_SIZE_X / 2.0),
            self.y + (consts::CARD_SIZE_Y / 2.0),
        ];
        let scale = (self.scale, self.scale);

        is_ok = self.card_back.execute(|image| {
            window.draw_ex(
                &image.area().with_center((pos[0], pos[1])),
                Img(&image),
                Transform::scale(scale),
                7,
            );
            Ok(())
        });
        if !is_ok.is_ok() || !self.cards_played == 0 {
            return is_ok;
        }

        is_ok = self.previous_card.draw(window);

        if is_ok.is_ok() && self.cards_played > 1 {
            self.last_card.draw(window);
        }
        is_ok
    }
}
