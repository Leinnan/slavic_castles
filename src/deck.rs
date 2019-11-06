use crate::card::Card;
use crate::consts::*;
use crate::resource::*;
use rand::prelude::*;
use std::fmt;

pub struct Deck {
    pub cards: Vec<Card>,
    cards_collections: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        cards.push(Card {
            id: 1,
            cost_resource: ResourceType::Soldiers,
            cost_amount: 3,
            tower_growth: 0,
            walls_growth: 0,
            damage: 3,
            production_resource: ResourceType::Soldiers,
            production_change: 0,
        });
        cards.push(Card {
            id: 2,
            cost_resource: ResourceType::Tools,
            cost_amount: 4,
            tower_growth: 1,
            walls_growth: 3,
            damage: 1,
            production_resource: ResourceType::Soldiers,
            production_change: 0,
        });
        cards.push(Card {
            id: 3,
            cost_resource: ResourceType::Soldiers,
            cost_amount: 3,
            tower_growth: 0,
            walls_growth: 0,
            damage: 5,
            production_resource: ResourceType::Soldiers,
            production_change: 0,
        });
        cards.push(Card {
            id: 4,
            cost_resource: ResourceType::Soldiers,
            cost_amount: 7,
            tower_growth: 0,
            walls_growth: 0,
            damage: 1,
            production_resource: ResourceType::Soldiers,
            production_change: 2,
        });
        cards.push(Card {
            id: 5,
            cost_resource: ResourceType::Tools,
            cost_amount: 10,
            tower_growth: 0,
            walls_growth: 3,
            damage: 0,
            production_resource: ResourceType::Tools,
            production_change: 2,
        });
        cards.push(Card {
            id: 6,
            cost_resource: ResourceType::Tools,
            cost_amount: 15,
            tower_growth: 10,
            walls_growth: 0,
            damage: 0,
            production_resource: ResourceType::Tools,
            production_change: 2,
        });
        cards.push(Card {
            id: 7,
            cost_resource: ResourceType::Soldiers,
            cost_amount: 12,
            tower_growth: 0,
            walls_growth: 0,
            damage: 10,
            production_resource: ResourceType::Soldiers,
            production_change: 0,
        });
        cards.push(Card {
            id: 8,
            cost_resource: ResourceType::Magic,
            cost_amount: 8,
            tower_growth: 4,
            walls_growth: 0,
            damage: 0,
            production_resource: ResourceType::Magic,
            production_change: 1,
        });
        cards.push(Card {
            id: 9,
            cost_resource: ResourceType::Magic,
            cost_amount: 1,
            tower_growth: 0,
            walls_growth: 1,
            damage: 0,
            production_resource: ResourceType::Magic,
            production_change: 0,
        });
        cards.push(Card {
            id: 10,
            cost_resource: ResourceType::Magic,
            cost_amount: 3,
            tower_growth: 0,
            walls_growth: 2,
            damage: 2,
            production_resource: ResourceType::Magic,
            production_change: 0,
        });
        cards.push(Card {
            id: 11,
            cost_resource: ResourceType::Magic,
            cost_amount: 6,
            tower_growth: 2,
            walls_growth: 3,
            damage: 0,
            production_resource: ResourceType::Magic,
            production_change: 1,
        });

        let mut deck = Deck {
            cards: Vec::new(),
            cards_collections: cards,
        };
        deck.fill_deck();
        deck
    }

    pub fn replace_card(&mut self, card_nr: i32) {
        let mut rng = thread_rng();
        let i = rng.gen_range(0, self.cards_collections.len());
        self.cards[card_nr as usize] = self.cards_collections[i];
    }

    pub fn fill_deck(&mut self) {
        self.cards = Vec::new();
        let mut rng = thread_rng();
        for _x in 0..CARDS_IN_DECK {
            let i = rng.gen_range(0, self.cards_collections.len());
            let card = self.cards_collections[i];
            self.cards.push(card);
        }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Deck: \n {0},\n {1},\n {2},\n {3}",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3],
        )
    }
}
