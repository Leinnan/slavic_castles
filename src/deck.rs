use crate::card::Card;
use crate::resource::*;
use std::fmt;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let card_1 = Card {
            id: 1,
            cost_resource: ResourceType::Soldiers,
            cost_amount: 3,
            tower_growth: 0,
            walls_growth: 0,
            damage: 3,
            production_resource: ResourceType::Soldiers,
            production_change: 0,
        };
        let card_2 = Card {
            id: 2,
            cost_resource: ResourceType::Tools,
            cost_amount: 4,
            tower_growth: 1,
            walls_growth: 3,
            damage: 1,
            production_resource: ResourceType::Soldiers,
            production_change: 0,
        };
        let card_3 = Card {
            id: 3,
            cost_resource: ResourceType::Soldiers,
            cost_amount: 3,
            tower_growth: 0,
            walls_growth: 0,
            damage: 5,
            production_resource: ResourceType::Soldiers,
            production_change: 0,
        };
        let card_4 = Card {
            id: 4,
            cost_resource: ResourceType::Soldiers,
            cost_amount: 7,
            tower_growth: 0,
            walls_growth: 0,
            damage: 1,
            production_resource: ResourceType::Soldiers,
            production_change: 2,
        };

        let cards = vec![card_1, card_2, card_3, card_4];

        Deck { cards }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Deck: \n {0},\n {1},\n {2},\n {3}",
            self.cards[0], self.cards[1], self.cards[2],self.cards[3],
        )
    }
}
