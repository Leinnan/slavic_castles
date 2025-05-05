use bevy::{prelude::Component, reflect::Reflect};
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub avatar_id: i32,
    pub name: String,
}

impl Profile {
    pub fn get_avatar_path(&self) -> String {
        Self::format_avatar_path(self.avatar_id)
    }

    pub fn save_profile(self, pkv: &mut PkvStore) {
        pkv.set("profile_info", &self)
            .expect("Failed to store profile");
    }

    pub fn format_avatar_path(id: i32) -> String {
        format!("avatars/{}.png", id)
    }
}

pub trait ProfileProvider {
    fn get_profile(&self) -> Option<Profile>;
    fn has_profile(&self) -> bool {
        self.get_profile().is_some()
    }
}

impl ProfileProvider for PkvStore {
    fn get_profile(&self) -> Option<Profile> {
        self.get::<Profile>("profile_info").ok()
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            avatar_id: 1,
            name: "Wojmir".to_owned(),
        }
    }
}
