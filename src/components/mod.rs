use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;
pub mod card_display;
pub mod player_ui;

pub trait ObserverExtension {
    fn observe_in_child<E: Event, B: Bundle, M>(
        &mut self,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self;
}

impl ObserverExtension for EntityCommands<'_> {
    fn observe_in_child<E: Event, B: Bundle, M>(
        &mut self,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self {
        self.with_children(|p| {
            let entity = p.target_entity();
            p.spawn(Observer::new(system).with_entity(entity));
        })
    }
}
