use bevy::{prelude::Component, reflect::Reflect};
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub avatar_id: i32,
    pub name: String,
}

impl Profile {
    pub fn get_avatar_path(self) -> String {
        get_avatar_path(self.avatar_id)
    }

    pub fn save_profile(self, pkv: &mut PkvStore) {
        pkv.set("profile_info", &self)
            .expect("Failed to store profile");
    }
}

pub fn get_profile(pkv: &PkvStore) -> Option<Profile> {
    if let Ok(profile) = pkv.get::<Profile>("profile_info") {
        Some(profile)
    } else {
        None
    }
}

pub fn has_profile(pkv: &PkvStore) -> bool {
    get_profile(pkv).is_some()
}

pub fn get_avatar_path(id: i32) -> String {
    format!("avatars/{}.png", id)
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            avatar_id: 1,
            name: "Wojmir".to_owned(),
        }
    }
}
