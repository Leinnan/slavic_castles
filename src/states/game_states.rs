use crate::base_systems::buttons::ButtonsPlugin;
use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{
    game::GamePlugin, loading::LoadingPlugin, menu::MenuPlugin,
    profile_selection::ProfileSelectionPlugin,
};

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum GameState {
    #[default]
    AssetsLoading,
    Menu,
    ProfileEdit,
    Game,
}

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ButtonsPlugin)
            .add(MenuPlugin)
            .add(GamePlugin)
            .add(LoadingPlugin)
            .add(ProfileSelectionPlugin)
            .add(crate::components::card_display::CardPlugin)
    }
}
