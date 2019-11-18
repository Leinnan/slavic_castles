use crate::consts;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Eq, Hash, Copy, Debug, Clone, Deserialize, Serialize)]
pub enum ResourceType {
    Tools,
    Magic,
    Soldiers,
}

pub struct Resource {
    pub amount: i32,
    pub production: i32,
}

impl Resource {
    pub fn new() -> Resource {
        Resource {
            amount: consts::BASE_RESOURCE_AMOUNT,
            production: consts::BASE_RESOURCE_PRODUCTION,
        }
    }

    pub fn reset(&mut self) {
        self.amount = consts::BASE_RESOURCE_AMOUNT;
        self.production = consts::BASE_RESOURCE_PRODUCTION;
    }

    pub fn produce(&mut self) {
        self.change_amount(self.production);
    }

    pub fn change_amount(&mut self, diff: i32) {
        self.amount += diff;
        if self.amount <= 0 {
            self.amount = 0;
        }
    }

    pub fn change_production(&mut self, diff: i32) {
        self.production += diff;
        if self.production <= 0 {
            self.production = 1;
        }
    }

    pub fn resource_name(res_type: &ResourceType) -> String {
        match res_type {
            ResourceType::Magic => "Magic".to_string(),
            ResourceType::Tools => "Tools".to_string(),
            ResourceType::Soldiers => "Soldiers".to_string(),
            _ => "NONE".to_string(),
        }
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},({})", self.amount, self.production)
    }
}
