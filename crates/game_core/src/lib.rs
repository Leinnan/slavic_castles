#[cfg(feature = "bevy")]
use bevy::prelude::*;

pub mod consts;
pub mod data;

#[cfg(feature = "bevy")]
pub struct GameCorePlugin;

#[cfg(feature = "bevy")]
impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<crate::data::supply::PlayerSupply>()
            .register_type::<crate::data::player::PlayerHealth>()
            .register_type::<crate::data::card::Card>();
    }
}
