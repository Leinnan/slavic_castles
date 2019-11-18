use crate::resource::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Eq, Hash, Copy, Debug, Clone, Deserialize, Serialize)]
pub enum EffectType {
    ProductionChange(ResourceType, i32), // dont use multiple production change per user
    Damage(i32, bool),                   // if true ignores wall
    ResourceChange(ResourceType, i32),
    TowerGrowth(i32),
    WallsGrowth(i32),
    None,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CardEffect {
    pub affects_user: bool,
    pub effect_type: EffectType,
}

impl CardEffect {
    pub fn Empty() -> CardEffect {
        CardEffect {
            affects_user: true,
            effect_type: EffectType::None,
        }
    }
}

impl fmt::Display for CardEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = match self.effect_type {
            EffectType::ProductionChange(resource, amount) => format!(
                "{} to {} production",
                amount,
                Resource::resource_name(&resource)
            ),
            EffectType::Damage(amount, ignore_wall) if ignore_wall => {
                format!("{} damage(ignores wall)", amount)
            }
            EffectType::Damage(amount, ignore_wall) => format!("{} damage", amount),
            EffectType::ResourceChange(resource, amount) => {
                format!("Adds {} of {}", amount, Resource::resource_name(&resource))
            }
            EffectType::TowerGrowth(growth) => format!("Adds {0} HP", growth,),
            EffectType::WallsGrowth(growth) => format!("Adds {0} shield", growth,),
            EffectType::None => String::new(),
        };
        if self.effect_type != EffectType::None {
            if self.affects_user {
                //result.push_str("to you");
            } else {
                result.push_str(" to enemy");
            }
        }
        write!(f, "{}", result)
    }
}
