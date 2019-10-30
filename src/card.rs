use crate::resource::*;
use std::collections::HashMap;
use std::fmt;

pub struct Card {
    pub id: i32,
    pub name: String,
    pub cost_resource: ResourceType,
    pub cost_amount: i32,
    pub tower_growth: i32,
    pub walls_growth: i32,
    pub damage_for_enemy: i32,
}

impl Card {
    pub fn can_aford(&self, resources: &HashMap<ResourceType, Resource>) -> bool {
        resources[&self.cost_resource].amount >= self.cost_amount
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{0}: Cost: {1}, Growth: {2}+{3}, Damage: {4}",
            self.name,
            self.cost_amount,
            self.tower_growth,
            self.walls_growth,
            self.damage_for_enemy,
        )
    }
}
