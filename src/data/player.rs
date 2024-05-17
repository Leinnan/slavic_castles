use bevy::{prelude::Component, reflect::Reflect};
use serde::{Deserialize, Serialize};

use super::consts;

#[derive(Component, Copy, Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct Player {
    pub tower_hp: i32,
    pub walls_hp: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            tower_hp: consts::BASE_TOWER_HP,
            walls_hp: consts::BASE_WALLS_HP,
        }
    }
}

impl Player {
    pub fn is_alive(&self) -> bool {
        self.tower_hp > 0
    }

    pub fn has_max_possible_tower(&self) -> bool {
        self.tower_hp >= consts::MAX_TOWER_HP
    }

    pub fn make_tower_higher(&mut self, amount: i32) {
        self.tower_hp = (self.tower_hp + amount).min(consts::MAX_TOWER_HP);
    }

    pub fn make_walls_higher(&mut self, amount: i32) {
        self.walls_hp = (self.walls_hp + amount).min(consts::MAX_WALLS_HP);
    }

    pub fn give_damage(&mut self, amount: i32, ignore_wall: bool) {
        if ignore_wall {
            self.tower_hp -= amount;
        } else if self.walls_hp < amount {
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
}
