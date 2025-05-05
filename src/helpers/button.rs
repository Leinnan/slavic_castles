use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<ButtonReleased>()
        .add_systems(Update, button_click_system.in_set(bevy::input::InputSystem));
}

/// Event triggered when Interaction changes from `Interaction::Pressed`
/// to `Interaction::Hovered` for entity that has GameButton component.
/// It is using the observer pattern for events.
#[derive(Event, Reflect)]
pub struct ButtonReleased;

#[derive(QueryFilter)]
pub struct ChangedButtonsFilter {
    _a: Changed<Interaction>,
    _b: With<Button>,
}

fn button_click_system(
    touches: Res<Touches>,
    mut last_clicked: Local<Option<Entity>>,
    interaction_query: Query<(Entity, &Interaction), ChangedButtonsFilter>,
    mut commands: Commands,
) {
    let any_input_released = touches.any_just_released();
    for (entity, interaction) in &interaction_query {
        let was_hovered = any_input_released || *interaction == Interaction::Hovered;
        match interaction {
            Interaction::Pressed => {
                *last_clicked = Some(entity);
            }
            _ if was_hovered && last_clicked.as_ref().is_some_and(|s| s.eq(&entity)) => {
                commands.trigger_targets(ButtonReleased, entity);
            }
            Interaction::None if last_clicked.as_ref().is_some_and(|s| s.eq(&entity)) => {
                *last_clicked = None;
            }
            _ => continue,
        }
    }
}
