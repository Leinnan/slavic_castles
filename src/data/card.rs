use super::resource::ResourceType;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect, Default)]
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

#[derive(PartialEq, Eq, Hash, Copy, Debug, Clone, Deserialize, Serialize, Default, Reflect)]
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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Default, Reflect)]
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
