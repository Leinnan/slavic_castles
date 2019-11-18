use crate::card_effect::*;
use crate::resource::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: i32,
    pub cost_resource: ResourceType,
    pub effects: [CardEffect; 3],
    pub cost_amount: i32,
    pub tower_growth: i32,
    pub walls_growth: i32,
    pub damage: i32,
    pub production_resource: ResourceType,
    pub production_change: i32,
}

impl Card {
    pub fn can_aford(&self, resources: &HashMap<ResourceType, Resource>) -> bool {
        resources[&self.cost_resource].amount >= self.cost_amount
    }

    pub fn production_change(&self, is_user: bool) -> (ResourceType,i32) {
        let mut production_change = (ResourceType::Magic,0i32);
        for el in self.effects.iter() {
            match el.effect_type {
                EffectType::ProductionChange(effect_type,amount) if el.affects_user == is_user => {
                    production_change.0 = effect_type;
                    production_change.1 += amount;
                },
                _ => {},
            }
        }
        production_change
    }

    pub fn damage(&self, is_user: bool) -> (i32,bool) {
        let mut damage = (0i32,false);
        for el in self.effects.iter() {
            match el.effect_type {
                EffectType::Damage(amount, ignore_wall) if el.affects_user == is_user => {
                    damage.0 += amount;
                    damage.1 = ignore_wall;
                },
                _ => {},
            }
        }
        damage
    }

    pub fn tower_growth(&self, is_user: bool) -> i32 {
        let mut growth = 0i32;
        for el in self.effects.iter() {
            match el.effect_type {
                EffectType::TowerGrowth(amount) if el.affects_user == is_user => {
                    growth += amount;
                },
                _ => {},
            }
        }
        growth
    }

    pub fn walls_growth(&self, is_user: bool) -> i32 {
        let mut growth = 0i32;
        for el in self.effects.iter() {
            match el.effect_type {
                EffectType::WallsGrowth(amount) if el.affects_user == is_user => {
                    growth += amount;
                },
                _ => {},
            }
        }
        growth
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        if self.cost_amount > 0 {
            result.push_str(&format!(
                "Cost({0}): {1}",
                Resource::resource_name(&self.cost_resource),
                self.cost_amount
            ));
        }
        if self.tower_growth + self.walls_growth > 0 {
            result.push_str(&format!(
                ", Growth: {0}+{1}",
                self.tower_growth, self.walls_growth,
            ));
        }
        if self.damage > 0 {
            result.push_str(&format!(", Damage: {0}", self.damage));
        }
        if self.production_change != 0 {
            result.push_str(&format!(
                ", Production({0}): {1}",
                Resource::resource_name(&self.production_resource),
                self.production_change,
            ));
        }
        write!(f, "{}", result)
    }
}
