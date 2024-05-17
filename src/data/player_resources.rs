use bevy::{prelude::Component, reflect::Reflect};
use serde::{Deserialize, Serialize};

use super::{
    card::Card,
    resource::{CastleResource, ResourceType},
};

#[derive(Component, Default, Reflect, Serialize, Deserialize)]
pub struct PlayerResources {
    pub tools: CastleResource,
    pub magic: CastleResource,
    pub soldiers: CastleResource,
}

impl PlayerResources {
    /// Call the produce method on each resource type
    pub fn update_resources(&mut self) {
        self.tools.produce();
        self.magic.produce();
        self.soldiers.produce();
    }
    pub fn change_resource_amount(&mut self, res_type: ResourceType, amount: i32) {
        match res_type {
            ResourceType::Magic => self.magic.change_amount(amount),
            ResourceType::Tools => self.tools.change_amount(amount),
            ResourceType::Soldiers => self.soldiers.change_amount(amount),
        };
    }

    pub fn change_resource_production(&mut self, res_type: ResourceType, amount: i32) {
        match res_type {
            ResourceType::Magic => self.magic.change_production(amount),
            ResourceType::Tools => self.tools.change_production(amount),
            ResourceType::Soldiers => self.soldiers.change_production(amount),
        };
    }

    pub fn get(&self, res_type: ResourceType) -> &CastleResource {
        match res_type {
            ResourceType::Magic => &self.magic,
            ResourceType::Tools => &self.tools,
            ResourceType::Soldiers => &self.soldiers,
        }
    }

    pub fn print(&self) -> String {
        format!(
            "Tools: {}\nMagic: {}\nSoldiers: {}",
            self.tools, self.magic, self.soldiers
        )
    }

    pub fn can_afford_card(&self, card: &Card) -> bool {
        let current_amount = self.get(card.cost_resource).amount;
        current_amount >= card.cost_amount
    }
}
