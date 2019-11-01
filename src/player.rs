use crate::deck::Deck;
use crate::resource::*;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Copy, Debug, Clone)]
pub enum PlayerNumer {
    First,
    Second,
}

pub struct Player {
    pub human: bool,
    pub alive: bool,
    pub tower_hp: i32,
    pub walls_hp: i32,
    pub resources: HashMap<ResourceType, Resource>,
    pub deck: Deck,
    active: bool
}

impl Player {
    pub fn new(active: bool) -> Player {
        let mut resources = HashMap::new();
        resources.insert(ResourceType::Tools, Resource::new());
        resources.insert(ResourceType::Magic, Resource::new());
        resources.insert(ResourceType::Soldiers, Resource::new());
        Player {
            human: true,
            alive: true,
            tower_hp: 20,
            walls_hp: 15,
            resources: resources,
            deck: Deck::new(),
            active: active,
        }
    }
    
    pub fn is_active(&self) -> bool {
        self.active
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
    }

    pub fn make_walls_higher(&mut self, amount: i32) {
        self.walls_hp += amount;
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

        if self.tower_hp == 0 {
            self.alive = false;
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
        self.deck.replace_card(nr);
        self.active = false;
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
            write!( f, "First")
        }
        else {
            write!( f, "Second")
        }
    }
}
