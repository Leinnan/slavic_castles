use crate::states::consts::FULL_SIZE_PERCENT;
use bevy::prelude::{AlignContent, AlignItems, FlexDirection, JustifyContent, Node, default};

pub mod consts;
pub mod game;
pub mod game_states;
pub mod loading;
pub mod menu;
pub mod profile_selection;

pub(super) fn root_node() -> Node {
    Node {
        height: FULL_SIZE_PERCENT,
        width: FULL_SIZE_PERCENT,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        align_content: AlignContent::Center,
        flex_direction: FlexDirection::Column,
        ..default()
    }
}
