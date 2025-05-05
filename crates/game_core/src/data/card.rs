use super::resource::ResourceType;
#[cfg(feature = "bevy")]
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct Card {
    pub name: String,
    pub id: i32,
    pub cost_amount: i32,
    pub cost_resource: ResourceType,
    pub effects: Vec<CardEffect>,
}

impl Card {
    pub fn get_sound_asset(&self) -> String {
        format!(
            "snd/card_{}.ogg",
            self.cost_resource.to_string().to_lowercase()
        )
    }

    pub fn production_change(&self, is_user: bool) -> (ResourceType, i32) {
        let mut production_change = (ResourceType::Magic, 0i32);
        for el in self.effects.iter() {
            match el.effect_type {
                EffectType::ProductionChange(effect_type, amount) if el.affects_user == is_user => {
                    production_change.0 = effect_type;
                    production_change.1 += amount;
                }
                _ => {}
            }
        }
        production_change
    }

    pub fn resource_amount_change(&self, is_user: bool) -> (ResourceType, i32) {
        let mut change = (ResourceType::Magic, 0i32);
        for el in self.effects.iter() {
            match el.effect_type {
                EffectType::ResourceChange(effect_type, amount) if el.affects_user == is_user => {
                    change.0 = effect_type;
                    change.1 += amount;
                }
                _ => {}
            }
        }
        change
    }

    pub fn damage(&self, is_user: bool) -> (i32, bool) {
        let mut damage = (0i32, false);
        for el in self.effects.iter() {
            match el.effect_type {
                EffectType::Damage(amount, ignore_wall) if el.affects_user == is_user => {
                    damage.0 += amount;
                    damage.1 = ignore_wall;
                }
                _ => {}
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
                }
                _ => {}
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
                }
                _ => {}
            }
        }
        growth
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Debug, Clone, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub enum EffectType {
    /// dont use multiple production change per user
    ProductionChange(ResourceType, i32),
    /// if true ignores wall
    Damage(i32, bool),
    ResourceChange(ResourceType, i32),
    TowerGrowth(i32),
    WallsGrowth(i32),
    #[default]
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct CardEffect {
    pub affects_user: bool,
    pub effect_type: EffectType,
}

impl CardEffect {
    pub fn empty() -> CardEffect {
        CardEffect {
            affects_user: true,
            effect_type: EffectType::None,
        }
    }
}

impl fmt::Display for CardEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = match self.effect_type {
            EffectType::ProductionChange(resource, amount) => {
                let sign = if amount > 0 { "Increase" } else { "Decrease" };
                let target = if self.affects_user { "" } else { " enemy" };

                return write!(
                    f,
                    "{}{} prod of {} by {}",
                    sign,
                    target,
                    &resource,
                    amount.abs()
                );
            }
            EffectType::Damage(amount, ignore_wall) => {
                let ignore_wall = if ignore_wall { "(ignores shield)" } else { "" };
                let target = if self.affects_user {
                    "Takes"
                } else {
                    "Deliver"
                };
                return write!(f, "{} {} damage{}", target, amount, ignore_wall);
            }
            EffectType::ResourceChange(resource, amount) => {
                let sign = if amount > 0 { "Gives" } else { "Takes" };
                let target_suffix = if self.affects_user {
                    ""
                } else if amount > 0 {
                    " to enemy"
                } else {
                    " from enemy"
                };

                return write!(
                    f,
                    "{} {} of {}{}",
                    sign,
                    amount.abs(),
                    &resource,
                    target_suffix
                );
            }
            EffectType::TowerGrowth(growth) => {
                let sign = if growth > 0 { "+" } else { "-" };
                format!("{}{} Health", sign, growth)
            }
            EffectType::WallsGrowth(growth) => {
                let sign = if growth > 0 { "+" } else { "-" };
                format!("{}{} Shield", sign, growth)
            }
            EffectType::None => {
                return write!(f, "");
            }
        };
        if !self.affects_user {
            result.push_str(" to enemy");
        }
        write!(f, "{}", result)
    }
}
