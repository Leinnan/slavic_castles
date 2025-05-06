use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use game_core::consts::BASE_RESOURCE_AMOUNT;
use game_core::consts::CARDS_IN_DECK;
use game_core::data::card::Card;
use game_core::data::supply::PlayerSupply;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize, Reflect, Component, Deref, DerefMut, Default, Debug)]
pub struct HandCards(pub Vec<Card>);

#[derive(QueryData)]
#[query_data(derive(Debug))]
pub struct HandQueryRead {
    pub cards: &'static HandCards,
    pub supply: &'static PlayerSupply,
}

pub struct CardInfo<'a> {
    pub card: &'a Card,
    pub index: usize,
    pub can_afford: bool,
}

impl CardInfo<'_> {
    pub fn color(&self) -> Color {
        if self.can_afford {
            Color::WHITE
        } else {
            Color::WHITE.darker(0.4)
        }
    }

    pub fn image_path(&self) -> String {
        format!("cards/{}.png", self.card.id)
    }
}

impl HandQueryReadItem<'_> {
    pub fn card_info_array(&self) -> Vec<CardInfo> {
        self.cards
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let can_afford = self.supply.can_afford_card(c);
                CardInfo {
                    card: c,
                    index: i,
                    can_afford,
                }
            })
            .collect()
    }
}

#[derive(
    serde::Deserialize, bevy::asset::Asset, Deref, Debug, DerefMut, Reflect, Default, Clone,
)]
pub struct DeckAsset(pub Vec<Card>);

impl HandCards {
    pub fn replace_card(&mut self, card_nr: usize, resources: &PlayerSupply, deck: &[Card]) {
        let mut rng = thread_rng();
        let mut finded = false;

        let mut card = &deck[0];
        for attempt in 0..9 {
            let i: usize = rng.gen::<usize>() % deck.len();
            card = &deck[i];
            let max_cost_amount = resources.get(card.cost_resource).amount + 2;
            let card_already_in_deck = attempt > 5 || self.iter().any(|c| c.id == card.id);
            if card.cost_amount <= max_cost_amount && !card_already_in_deck {
                self[card_nr] = card.clone();
                finded = true;
                break;
            }
        }
        if !finded {
            self[card_nr] = card.clone();
        }
    }

    pub fn rnd(&self) -> usize {
        let mut rng = thread_rng();
        let i: usize = rng.gen::<usize>() % self.len();
        i
    }

    pub fn generate_random(cards: &[Card]) -> Self {
        let mut deck = HandCards::default();
        let mut rng = thread_rng();
        let max_cost_amount = BASE_RESOURCE_AMOUNT * 120 / 100;
        for _x in 0..CARDS_IN_DECK {
            let mut found = false;

            let mut card: &Card = &cards[0];
            for attempt in 0..9 {
                let i: usize = rng.gen::<usize>() % cards.len();
                card = &cards[i];
                let card_already_in_deck = attempt > 5 || deck.iter().any(|c| c.id == card.id);
                if card.cost_amount <= max_cost_amount && !card_already_in_deck {
                    deck.push(card.clone());
                    found = true;
                    break;
                }
            }
            if !found {
                deck.push(card.clone());
            }
        }
        deck
    }
}
