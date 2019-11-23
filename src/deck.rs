use crate::card::Card;
use crate::card_effect::*;
use crate::consts::*;
use crate::resource::*;
use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Col, Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt;
use std::str;

#[derive(Serialize, Deserialize)]
pub struct Deck {
    pub cards: Vec<Card>,
    cards_collections: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck {
            cards: Vec::new(),
            cards_collections: Deck::load_deck(),
        };
        deck.fill_deck();
        deck
    }

    pub fn replace_card(&mut self, card_nr: i32, resources: &HashMap<ResourceType, Resource>) {
        let mut rng = thread_rng();
        let mut finded = false;

        let mut card = self.cards_collections[0];
        for attempt in 0..9 {
            let i = rng.gen_range(0, self.cards_collections.len());
            card = self.cards_collections[i];
            let max_cost_amount = resources[&card.cost_resource].amount * 120 / 100;
            let card_already_in_deck = attempt > 5 || self.cards.iter().any(|&c| c.id == card.id);
            if card.cost_amount <= max_cost_amount && !card_already_in_deck {
                self.cards[card_nr as usize] = card;
                finded = true;
                break;
            }
        }
        if !finded {
            self.cards[card_nr as usize] = card;
        }
    }

    pub fn fill_deck(&mut self) {
        self.cards = Vec::new();
        let mut rng = thread_rng();
        let max_cost_amount = BASE_RESOURCE_AMOUNT * 120 / 100;
        for _x in 0..CARDS_IN_DECK {
            let mut finded = false;

            let mut card = self.cards_collections[0];
            for attempt in 0..9 {
                let i = rng.gen_range(0, self.cards_collections.len());
                card = self.cards_collections[i];
                let card_already_in_deck =
                    attempt > 5 || self.cards.iter().any(|&c| c.id == card.id);
                if card.cost_amount <= max_cost_amount && !card_already_in_deck {
                    self.cards.push(card);
                    finded = true;
                    break;
                }
            }
            if !finded {
                self.cards.push(card);
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_deck() -> Vec<Card> {
        let mut json_file = Asset::new(quicksilver::load_file("deck.json"));
        let mut buf: Vec<u8> = Vec::new();
        json_file.execute(|f| {
            buf = f.clone();
            Ok(())
        });
        let result_json = str::from_utf8(&buf).unwrap();
        serde_json::from_str(result_json).unwrap()
    }

    #[cfg(target_arch = "wasm32")]
    fn load_deck() -> Vec<Card> {
        let buf = DECK_JSON.to_vec();
        let result_json = str::from_utf8(&buf).unwrap();
        serde_json::from_str(result_json).unwrap()
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Deck: \n {0},\n {1},\n {2},\n {3},\n {4}",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4],
        )
    }
}
