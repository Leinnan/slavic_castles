use crate::card::Card;
use crate::card_effect::*;
use crate::consts::*;
use crate::resource::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt;

pub struct Deck {
    pub cards: Vec<Card>,
    cards_collections: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        let none = CardEffect::Empty();
        cards.push(Card {
            id: 1,
            cost_resource: ResourceType::Soldiers,
            cost_amount: 3,
            effects: [
                CardEffect {
                    affects_user: false,
                    effect_type: EffectType::Damage(3, false),
                },
                none,
                none,
            ],
        });
        cards.push(Card {
            id: 2,
            cost_resource: ResourceType::Tools,
            cost_amount: 4,
            effects: [
                CardEffect {
                    affects_user: false,
                    effect_type: EffectType::Damage(1, false),
                },
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::WallsGrowth(3),
                },
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::TowerGrowth(1),
                },
            ],
        });
        cards.push(Card {
            id: 3,
            cost_resource: ResourceType::Soldiers,
            cost_amount: 3,
            effects: [
                CardEffect {
                    affects_user: false,
                    effect_type: EffectType::Damage(5, false),
                },
                none,
                none,
            ],
        });
        cards.push(Card {
            id: 4,
            cost_resource: ResourceType::Soldiers,
            cost_amount: 7,
            effects: [
                CardEffect {
                    affects_user: false,
                    effect_type: EffectType::Damage(1, false),
                },
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::ProductionChange(ResourceType::Soldiers, 2),
                },
                none,
            ],
        });
        cards.push(Card {
            id: 5,
            cost_resource: ResourceType::Tools,
            cost_amount: 10,
            effects: [
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::WallsGrowth(3),
                },
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::ProductionChange(ResourceType::Tools, 2),
                },
                none,
            ],
        });
        cards.push(Card {
            id: 6,
            cost_resource: ResourceType::Tools,
            cost_amount: 15,
            effects: [
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::TowerGrowth(3),
                },
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::ProductionChange(ResourceType::Tools, 2),
                },
                none,
            ],
        });
        cards.push(Card {
            id: 7,
            cost_resource: ResourceType::Soldiers,
            cost_amount: 12,
            effects: [
                CardEffect {
                    affects_user: false,
                    effect_type: EffectType::Damage(10, false),
                },
                none,
                none,
            ],
        });
        cards.push(Card {
            id: 8,
            cost_resource: ResourceType::Magic,
            cost_amount: 8,
            effects: [
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::TowerGrowth(4),
                },
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::ProductionChange(ResourceType::Magic, 1),
                },
                none,
            ],
        });
        cards.push(Card {
            id: 9,
            cost_resource: ResourceType::Magic,
            cost_amount: 1,
            effects: [
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::WallsGrowth(1),
                },
                none,
                none,
            ],
        });
        cards.push(Card {
            id: 10,
            cost_resource: ResourceType::Magic,
            cost_amount: 3,
            effects: [
                CardEffect {
                    affects_user: false,
                    effect_type: EffectType::Damage(2, true),
                },
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::WallsGrowth(2),
                },
                none,
            ],
        });
        cards.push(Card {
            id: 11,
            cost_resource: ResourceType::Magic,
            cost_amount: 6,
            effects: [
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::TowerGrowth(2),
                },
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::WallsGrowth(3),
                },
                CardEffect {
                    affects_user: true,
                    effect_type: EffectType::ProductionChange(ResourceType::Magic, 1),
                },
            ],
        });
        cards.push(Card {
            id: 12,
            cost_resource: ResourceType::Magic,
            cost_amount: 9,
            effects: [
                CardEffect {
                    affects_user: false,
                    effect_type: EffectType::Damage(5, true),
                },
                CardEffect {
                    affects_user: false,
                    effect_type: EffectType::ProductionChange(ResourceType::Magic, -1),
                },
                none,
            ],
        });
        // let cards_json = serde_json::to_string(&cards);
        // if cards_json.is_ok() {
        //     println!("{}",cards_json.unwrap());
        // }
        let mut deck = Deck {
            cards: Vec::new(),
            cards_collections: cards,
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
