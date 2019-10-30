use crate::resource::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub id: i32,
    pub cost_resource: ResourceType,
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
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::from(format!("{0}: ",self.id));
        if self.cost_amount > 0 {
            result.push_str(&format!("Cost({0}): {1}", Resource::resource_name(&self.cost_resource), self.cost_amount));
        }
        if self.tower_growth + self.walls_growth > 0 {
            result.push_str(&format!(", Growth: {0}+{1}", 
            self.tower_growth,
            self.walls_growth,
            ));
        }
        if self.damage > 0 {
            result.push_str(&format!(", Damage: {0}",self.damage));
        }
        if self.production_change != 0 {
            result.push_str(&format!(", Production({0}): {1}",
            Resource::resource_name(&self.production_resource),
            self.production_change,
            ));
        }
        write!(f,"{}",result)
    }
}
