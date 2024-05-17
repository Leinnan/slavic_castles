use bevy::prelude::*;
use bevy::reflect::Reflect;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use std::str;

use super::card::Card;
use super::consts::BASE_RESOURCE_AMOUNT;
use super::consts::CARDS_IN_DECK;

use crate::PlayerResources;

#[derive(Serialize, Deserialize, Reflect, Component)]
pub struct HandCards {
    pub cards: Vec<Card>,
}

#[derive(
    serde::Deserialize, bevy::asset::Asset, Deref, Debug, DerefMut, Reflect, Default, Clone,
)]
pub struct DeckAsset(pub Vec<Card>);

impl HandCards {
    pub fn replace_card(&mut self, card_nr: usize, resources: &PlayerResources, deck: &[Card]) {
        let mut rng = thread_rng();
        let mut finded = false;

        let mut card = &deck[0];
        for attempt in 0..9 {
            let i: usize = rng.gen::<usize>() % deck.len();
            card = &deck[i];
            let max_cost_amount = resources.get(card.cost_resource).amount + 2;
            let card_already_in_deck = attempt > 5 || self.cards.iter().any(|c| c.id == card.id);
            if card.cost_amount <= max_cost_amount && !card_already_in_deck {
                self.cards[card_nr] = card.clone();
                finded = true;
                break;
            }
        }
        if !finded {
            self.cards[card_nr] = card.clone();
        }
    }

    pub fn rnd(&self) -> usize {
        let mut rng = thread_rng();
        let i: usize = rng.gen::<usize>() % self.cards.len();
        i
    }

    pub fn generate_random(cards: &[Card]) -> Self {
        let mut deck = HandCards { cards: Vec::new() };
        let mut rng = thread_rng();
        let max_cost_amount = BASE_RESOURCE_AMOUNT * 120 / 100;
        for _x in 0..CARDS_IN_DECK {
            let mut finded = false;

            let mut card: &Card = &cards[0];
            for attempt in 0..9 {
                let i: usize = rng.gen::<usize>() % cards.len();
                card = &cards[i];
                let card_already_in_deck =
                    attempt > 5 || deck.cards.iter().any(|c| c.id == card.id);
                if card.cost_amount <= max_cost_amount && !card_already_in_deck {
                    deck.cards.push(card.clone());
                    finded = true;
                    break;
                }
            }
            if !finded {
                deck.cards.push(card.clone());
            }
        }
        deck
    }
}
