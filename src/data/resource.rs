use crate::data::consts;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Eq, Hash, Copy, Debug, Clone, Deserialize, Serialize, Default, Reflect)]
pub enum ResourceType {
    #[default]
    Tools,
    Magic,
    Soldiers,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ResourceType::Magic => "Magic",
                ResourceType::Tools => "Tools",
                ResourceType::Soldiers => "Soldiers",
            }
        )
    }
}

#[derive(PartialEq, Eq, Reflect, Serialize, Deserialize, Copy, Debug, Hash, Clone)]
pub struct CastleResource {
    pub amount: i32,
    pub production: i32,
}

impl Default for CastleResource {
    fn default() -> Self {
        CastleResource::new()
    }
}

impl CastleResource {
    pub fn new() -> CastleResource {
        CastleResource {
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
        } else if self.production > consts::MAX_PRODUCTION {
            self.production = consts::MAX_PRODUCTION;
        }
    }
}

impl fmt::Display for CastleResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},({})", self.amount, self.production)
    }
}
