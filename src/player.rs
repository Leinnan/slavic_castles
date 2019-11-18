use crate::consts;
use crate::deck::Deck;
use crate::card::Card;
use crate::resource::*;
use rand::prelude::*;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Copy, Debug, Clone)]
pub enum PlayerNumer {
    First,
    Second,
}

pub struct Player {
    human: bool,
    pub tower_hp: i32,
    pub walls_hp: i32,
    pub resources: HashMap<ResourceType, Resource>,
    pub deck: Deck,
    active: bool,
}

impl Player {
    pub fn new(active: bool, human: bool) -> Player {
        let mut resources = HashMap::new();
        resources.insert(ResourceType::Tools, Resource::new());
        resources.insert(ResourceType::Magic, Resource::new());
        resources.insert(ResourceType::Soldiers, Resource::new());
        Player {
            human: human,
            tower_hp: consts::BASE_TOWER_HP,
            walls_hp: consts::BASE_WALLS_HP,
            resources: resources,
            deck: Deck::new(),
            active: active,
        }
    }

    pub fn reset(&mut self, active: bool, human: bool) {
        self.resources
            .get_mut(&ResourceType::Magic)
            .unwrap()
            .reset();
        self.resources
            .get_mut(&ResourceType::Tools)
            .unwrap()
            .reset();
        self.resources
            .get_mut(&ResourceType::Soldiers)
            .unwrap()
            .reset();
        self.tower_hp = consts::BASE_TOWER_HP;
        self.walls_hp = consts::BASE_WALLS_HP;
        self.deck.fill_deck();
        self.active = active;
        self.human = human;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn is_human(&self) -> bool {
        self.human
    }

    pub fn is_alive(&self) -> bool {
        self.tower_hp > 0
    }

    pub fn has_max_possible_tower(&self) -> bool {
        self.tower_hp >= consts::MAX_TOWER_HP
    }

    pub fn change_resource_amount(&mut self, res_type: &ResourceType, amount: i32) {
        self.resources
            .get_mut(res_type)
            .unwrap()
            .change_amount(amount);
    }

    pub fn change_resource_production(&mut self, res_type: &ResourceType, amount: i32) {
        self.resources
            .get_mut(res_type)
            .unwrap()
            .change_production(amount);
    }

    pub fn make_tower_higher(&mut self, amount: i32) {
        self.tower_hp += amount;
        if self.tower_hp > consts::MAX_TOWER_HP {
            self.tower_hp = consts::MAX_TOWER_HP;
        }
    }

    pub fn make_walls_higher(&mut self, amount: i32) {
        self.walls_hp += amount;
        if self.walls_hp > consts::MAX_WALLS_HP {
            self.walls_hp = consts::MAX_WALLS_HP;
        }
    }

    pub fn give_damage(&mut self, amount: i32, ignore_wall: bool) {
        if ignore_wall {
            self.tower_hp -= amount;
        }

        if self.walls_hp < amount {
            self.tower_hp -= amount - self.walls_hp;
            self.walls_hp = 0;
        } else {
            self.walls_hp -= amount;
            if self.walls_hp < 0 {
                self.walls_hp = 0;
            }
        }

        if self.tower_hp < 0 {
            self.tower_hp = 0
        }
    }

    pub fn start_new_turn(&mut self) {
        self.resources
            .get_mut(&ResourceType::Tools)
            .unwrap()
            .produce();
        self.resources
            .get_mut(&ResourceType::Magic)
            .unwrap()
            .produce();
        self.resources
            .get_mut(&ResourceType::Soldiers)
            .unwrap()
            .produce();

        self.active = true;
    }

    pub fn replace_card(&mut self, nr: i32) {
        self.deck.replace_card(nr, &self.resources);
        self.active = false;
    }

    pub fn get_possible_move(&self) -> (i32, bool) {
        for i in 0..self.deck.cards.len() {
            if self.deck.cards[i].can_aford(&self.resources) {
                return (i as i32, false);
            }
        }

        let mut rng = thread_rng();
        (rng.gen_range(0, consts::CARDS_IN_DECK), true)
    }

    pub fn card_used(&mut self, card: &Card, is_user: bool) {
        if is_user {
            self.change_resource_amount(&card.cost_resource, -card.cost_amount);
        }
        let prod = card.production_change(is_user);
        self.change_resource_production(&prod.0, prod.1);
        let damage = card.damage(is_user);
        self.give_damage(damage.0, damage.1);
        self.make_tower_higher(card.tower_growth(is_user));
        self.make_walls_higher(card.walls_growth(is_user));
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Player: Life: {0}+{1}, Tools: {2}, Magic: {3}, Soldiers: {4}",
            self.tower_hp,
            self.walls_hp,
            self.resources[&ResourceType::Tools],
            self.resources[&ResourceType::Magic],
            self.resources[&ResourceType::Soldiers]
        )
    }
}

impl fmt::Display for PlayerNumer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self == &PlayerNumer::First {
            write!(f, "First")
        } else {
            write!(f, "Second")
        }
    }
}
