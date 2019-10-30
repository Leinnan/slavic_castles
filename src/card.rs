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
    pub damage_for_enemy: i32,
}

impl Card {
    pub fn can_aford(&self, resources: &HashMap<ResourceType, Resource>) -> bool {
        resources[&self.cost_resource].amount >= self.cost_amount
    }

    pub fn resource_name(&self) -> String {
        match self.cost_resource {
            ResourceType::Magic => "Magic".to_string(),
            ResourceType::Tools => "Tools".to_string(),
            ResourceType::Soldiers => "Soldiers".to_string(),
            _ => "NONE".to_string(),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{0}: Cost({5}): {1}, Growth: {2}+{3}, Damage: {4}",
            self.id,
            self.cost_amount,
            self.tower_growth,
            self.walls_growth,
            self.damage_for_enemy,
            self.resource_name(),
        )
    }
}
