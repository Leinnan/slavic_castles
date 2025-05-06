use crate::base_systems::turn_based::{CurrentActorToken, GameTurnSteps};
use crate::components::ObserverExtension;
use crate::data::deck::HandQueryRead;
use crate::states::game::{self, ActionTaken, GameObject, HelpDisplay, HumanPlayer};
use crate::states::game_states::GameState;
use crate::visual::window_changed_or_component_added;
use bevy::color::palettes::tailwind;
use bevy::ecs::system::SystemParam;
use bevy::{prelude::*, reflect::Reflect};
use game_core::data::card::Card;

const CARD_SIZE: Vec3 = Vec3::splat(0.9);

#[derive(Debug, Clone, Copy)]
pub enum CardAction {
    ReturnToDeck,
    Use,
    DropIt,
}
#[derive(Component, Reflect, Default, Clone)]
#[relationship_target(relationship = CardSlot)]
pub struct CardSlotPlace(Vec<Entity>);

#[derive(Component, Reflect, Default, Clone, Deref)]
#[require(GameObject, CardSlotPlace)]
pub struct CardSlotIndex(pub usize);

#[derive(Component, Reflect, Deref, Clone)]
#[relationship(relationship_target = CardSlotPlace)]
pub struct CardSlot(Entity);

#[derive(SystemParam, Deref)]
pub struct CardPlaces<'w, 's>(
    Query<
        'w,
        's,
        (
            Entity,
            &'static CardSlotIndex,
            Option<&'static CardSlotPlace>,
        ),
    >,
);

impl CardPlaces<'_, '_> {
    fn entity_by_index(&self, i: usize) -> Option<Entity> {
        self.iter().find(|(_, s, _)| s.0 == i).map(|(e, _, _)| e)
    }

    fn empty_or_first(&self) -> Option<CardSlot> {
        if let Some(e) = self
            .iter()
            .find(|(_, _, p)| !p.is_some_and(|e| !e.is_empty()))
        {
            Some(CardSlot(e.0))
        } else {
            self.iter().next().map(|e| CardSlot(e.0))
        }
    }
}

#[derive(Component, Reflect, Default, Clone, Copy)]
#[require(Pickable, ActionToPerform, StateScoped::<GameTurnSteps>(GameTurnSteps::ActionSelection))]
pub struct DraggableCard;

#[derive(Component, Reflect, Default, Clone, Copy)]
pub struct CanThrowCard;

#[derive(Component, Debug, PartialEq, Clone, Reflect, Deref)]
pub struct CardDisplay(pub Card);

#[derive(Component, Reflect, Default, Clone, Copy, Debug, PartialEq)]
pub enum ActionToPerform {
    Use,
    Discard,
    #[default]
    Nothing,
}

#[derive(Component, Reflect, Default, Clone, Copy, Debug, PartialEq)]
#[require(GameObject, Pickable)]
pub enum CardDropZone {
    Use,
    #[default]
    Discard,
}

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<CardDisplay>()
            .register_type::<CardSlot>()
            .register_type::<CardSlotPlace>()
            .register_type::<CardSlotIndex>()
            .register_type::<DraggableCard>()
            .register_type::<CardDropZone>()
            .register_type::<ActionToPerform>()
            .add_systems(
                Update,
                update_card_places.run_if(window_changed_or_component_added::<CardSlot>),
            )
            .add_systems(
                Update,
                (
                    cards_scale_system,
                    card_return_system,
                    sort_cards,
                    update_card_color,
                )
                    .run_if(in_state(GameState::Game)),
            )
            // .add_systems(
            //     OnEnter(GameTurnSteps::PerformAction),
            //     despawn_recursive_by_component::<CardDisplay>,
            // )
            .add_systems(OnEnter(GameState::Game), add_card_places)
            .add_systems(
                OnEnter(GameTurnSteps::ActionSelection),
                (add_cards).after(game::switch_player),
            )
            .add_observer(drag)
            .add_observer(start_drag)
            .add_observer(end_drag);
    }
}

fn drag(trigger: Trigger<Pointer<Drag>>, mut drag: Query<&mut Transform, With<DraggableCard>>) {
    let Ok(mut transform) = drag.get_mut(trigger.target()) else {
        return;
    };
    let drag = trigger.event();
    transform.translation.x += drag.delta.x; // Make the square follow the mouse
    transform.translation.y -= drag.delta.y;
    transform.translation.z = 150.0;
    let clamped = inline_tweak::tweak!(0.35);
    let speed = inline_tweak::tweak!(0.05);
    let target_rotation =
        Quat::from_rotation_z((drag.delta.x / inline_tweak::tweak!(-1.5)).clamp(-clamped, clamped));
    transform.rotation = transform.rotation.lerp(target_rotation, speed);
}

fn add_card_places(windows: Query<&Window>, mut commands: Commands) {
    let window = windows.single().expect("");
    let y_pos = -window.height() + inline_tweak::tweak!(-350.0);
    let min = inline_tweak::tweak!(0.5);
    let max = inline_tweak::tweak!(-0.5);
    let step = (max - min) / 4.0;
    for i in 0..5 {
        let offset = (inline_tweak::tweak!(50) * i) as f32;
        commands
            .spawn(
                Transform::from_xyz(inline_tweak::tweak!(-600.0) + offset, y_pos, i as f32 + 1.0)
                    .with_rotation(Quat::from_rotation_z(min + step * i as f32)),
            )
            .insert(Name::new(format!("Place{}", i)))
            .insert(GameObject)
            .insert(CardSlotIndex(i));
    }

    commands
        .spawn((
            Transform::from_xyz(0.0, 100.0, 0.0),
            Sprite {
                color: Color::WHITE.with_alpha(0.0),
                custom_size: Some(Vec2::new(900.0, 300.0)),
                ..default()
            },
            CardDropZone::Use,
        ))
        .observe_in_child(on_drag_over_card)
        .observe_in_child(on_drag_leave_card);

    commands
        .spawn((
            Sprite {
                color: tailwind::RED_300.with_alpha(0.0).into(),
                custom_size: Some(Vec2::new(300.0, 300.0)),
                ..default()
            },
            Transform::from_xyz(650.0, -350.0, 0.0),
            CardDropZone::Discard,
        ))
        .observe_in_child(on_drag_over_card)
        .observe_in_child(on_drag_leave_card);
}

fn update_card_color(
    mut q: Query<(&mut Sprite, &ActionToPerform, Option<&CanThrowCard>), Changed<ActionToPerform>>,
) {
    for (mut sprite, action, can_throw) in q.iter_mut() {
        sprite.color = match (action, can_throw.is_some()) {
            (ActionToPerform::Use, true) => tailwind::AMBER_300.into(),
            (ActionToPerform::Discard, _) => Color::linear_rgb(1.0, 0.6, 0.6),
            (_, false) => Color::WHITE.darker(0.4),
            (_, _) => Color::WHITE,
        };
    }
}

fn on_drag_over_card(
    trigger: Trigger<Pointer<DragEnter>>,
    q: Query<&CardDropZone>,
    mut qq: Query<(&mut ActionToPerform, Option<&CanThrowCard>)>,
) -> Result {
    let zone_type = q.get(trigger.target())?;
    info!("{zone_type:?}");
    let (mut action, usable) = qq.get_mut(trigger.dragged)?;
    let new_action = match zone_type {
        CardDropZone::Use if usable.is_some() => ActionToPerform::Use,
        CardDropZone::Discard => ActionToPerform::Discard,
        _ => ActionToPerform::Nothing,
    };
    let _ = action.set_if_neq(new_action);
    Ok(())
}

fn on_drag_leave_card(
    trigger: Trigger<Pointer<DragLeave>>,
    q: Query<&CardDropZone>,
    mut qq: Query<&mut ActionToPerform, With<DraggableCard>>,
) -> Result {
    let zone_type = q.get(trigger.target())?;
    info!("Leave {zone_type:?}");
    if let Ok(mut action) = qq.get_mut(trigger.dragged) {
        let _ = action.set_if_neq(ActionToPerform::Nothing);
    }
    Ok(())
}

fn add_cards(
    windows: Query<&Window>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    deck_q: Query<HandQueryRead, (With<HumanPlayer>, With<CurrentActorToken>)>,
    places: CardPlaces,
) -> Result {
    let Ok(hand) = deck_q.single() else {
        return Ok(());
    };
    let window = windows.single()?;
    let y_pos = -window.height() + inline_tweak::tweak!(-450.0);

    for card_info in hand.card_info_array().iter() {
        let Some(slot_e) = places.entity_by_index(card_info.index) else {
            continue;
        };
        let offset = (inline_tweak::tweak!(200) * card_info.index) as f32;
        let mut cmd = commands.spawn((
            Sprite {
                color: card_info.color(),
                image: asset_server.load(card_info.image_path()),
                ..default()
            },
            Transform::from_xyz(-350.0 + offset, y_pos - 300.0, card_info.index as f32 + 1.0)
                .with_scale(CARD_SIZE),
            Name::new(format!("Card Nr {}", card_info.index)),
            DraggableCard,
            CardDisplay(card_info.card.clone()),
            CardSlot(slot_e),
        ));
        if card_info.can_afford {
            cmd.insert(CanThrowCard);
        }
    }
    Ok(())
}

fn start_drag(
    trigger: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    mut help_query: Query<&mut Node, With<HelpDisplay>>,
) -> Result {
    for mut s in help_query.iter_mut() {
        s.display = Display::None;
    }
    let mut e = commands.get_entity(trigger.target())?;
    e.insert(ActionToPerform::Nothing)
        .try_remove::<CardSlot>()
        .insert(Pickable {
            is_hoverable: true,
            should_block_lower: false,
        });
    Ok(())
}

fn sort_cards(
    mut cards_q: Query<(Entity, &Transform, Option<&CardSlot>), With<DraggableCard>>,
    places: CardPlaces,
    mut commands: Commands,
) -> Result {
    let mut cards = cards_q
        .iter_mut()
        .map(|(e, transform, card)| (e, transform.translation.x, card))
        .collect::<Vec<_>>();

    cards.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for (i, (e, _x, slot)) in cards.into_iter().enumerate() {
        let Some(slot) = slot else {
            continue;
        };
        let Some(place_e) = places.entity_by_index(i) else {
            continue;
        };
        if **slot != place_e {
            // warn!("CHANGING FROM {slot} to {place_e}, index: {i}");
            commands.get_entity(e)?.insert(CardSlot(place_e));
        }
    }
    Ok(())
}
fn update_card_places(
    windows: Query<&Window>,
    mut query2: Query<(&mut Transform, &CardSlotIndex)>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let modifier = inline_tweak::tweak!(-30.0);
    let modifier2 = inline_tweak::tweak!(-100.0);
    let pos_modifiers = [modifier2, modifier, 0.0, modifier, modifier2];
    for (mut t, place) in query2.iter_mut() {
        t.translation.y = window.height() * inline_tweak::tweak!(-0.3)
            + inline_tweak::tweak!(1.0)
            + pos_modifiers[**place];
        t.translation.x =
            inline_tweak::tweak!(-360.0) + (**place as f32) * inline_tweak::tweak!(180.0);
    }
}

fn end_drag(
    trigger: Trigger<Pointer<DragEnd>>,
    cards_q: Query<(&CardDisplay, &ActionToPerform)>,
    mut commands: Commands,
    q: Query<Entity, With<CurrentActorToken>>,
    places: CardPlaces,
) -> Result {
    let Ok((card_display, action)) = cards_q.get(trigger.target()) else {
        return Ok(());
    };
    let mut ent = commands.get_entity(trigger.target())?;
    ent.insert(Pickable {
        is_hoverable: true,
        should_block_lower: true,
    });
    if let Some(t) = places.empty_or_first() {
        ent.insert(t);
    }
    let Ok(player_entity) = q.single() else {
        return Ok(());
    };
    let action_taken = match action {
        ActionToPerform::Use => ActionTaken::UseCard {
            card: (**card_display).clone(),
        },
        ActionToPerform::Discard => ActionTaken::DropCard {
            card: (**card_display).clone(),
        },
        ActionToPerform::Nothing => return Ok(()),
    };
    commands.get_entity(player_entity)?.insert(action_taken);
    Ok(())
}

fn card_return_system(
    mut cards_q: Query<(&mut Transform, &CardSlot)>,
    time: Res<Time>,
    places_q: Query<&Transform, Without<CardSlot>>,
) {
    let speed = inline_tweak::tweak!(20.0);
    let delta = (time.delta_secs() * speed).min(1.0);

    for (mut t, card) in cards_q.iter_mut() {
        let Ok(place_t) = places_q.get(**card) else {
            continue;
        };
        t.translation = t.translation.lerp(place_t.translation, delta);
        t.rotation = t.rotation.lerp(place_t.rotation, delta);
    }
}

fn cards_scale_system(
    mut cards_q: Query<(&mut Transform, Option<&CardSlot>), With<DraggableCard>>,
    time: Res<Time>,
) {
    let speed = inline_tweak::tweak!(20.0);
    let delta = (time.delta_secs() * speed).min(1.0);

    for (mut t, slot) in cards_q.iter_mut() {
        let dragged = slot.is_none();
        let target_scale = if dragged {
            inline_tweak::tweak!(1.1) * CARD_SIZE
        } else {
            CARD_SIZE
        };
        t.scale = t.scale.lerp(target_scale, delta);
    }
}
