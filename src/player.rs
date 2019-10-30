use crate::deck::Deck;
use crate::resource::*;
use std::collections::HashMap;
use std::fmt;

pub struct Player {
    pub human: bool,
    pub alive: bool,
    pub id: i32,
    pub tower_hp: i32,
    pub walls_hp: i32,
    pub resources: HashMap<ResourceType, Resource>,
    pub deck: Deck,
}

impl Player {
    pub fn new(new_id: i32) -> Player {
        let mut resources = HashMap::new();
        resources.insert(ResourceType::Tools, Resource::new());
        resources.insert(ResourceType::Magic, Resource::new());
        resources.insert(ResourceType::Soldiers, Resource::new());
        Player {
            human: true,
            alive: true,
            id: new_id,
            tower_hp: 20,
            walls_hp: 15,
            resources: resources,
            deck: Deck::new(),
        }
    }

    pub fn change_resource_amount(&mut self, res_type: &ResourceType, amount: i32) {
        self.resources
            .get_mut(res_type)
            .unwrap()
            .change_amount(amount);
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
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Player{0}: Life: {1}+{2}, Tools: {3}, Magic: {4}, Soldiers: {5}",
            self.id,
            self.tower_hp,
            self.walls_hp,
            self.resources[&ResourceType::Tools],
            self.resources[&ResourceType::Magic],
            self.resources[&ResourceType::Soldiers]
        )
    }
}
