use bevy::color::palettes::tailwind;
use bevy::{prelude::*, reflect::Reflect};

use crate::base_systems::turn_based::{CurrentActorToken, GameTurnSteps};
use crate::data::deck::HandQueryRead;
use crate::helpers::despawn_recursive_by_component;
use crate::states::game::{self, ActionTaken, GameObject, HelpDisplay, HumanPlayer};
use crate::states::game_states::GameState;
use crate::visual::window_changed_or_component_added;
use game_core::data::card::Card;

const CARD_SIZE: Vec3 = Vec3::splat(0.95);

#[derive(Debug, Clone, Copy)]
pub enum CardAction {
    ReturnToDeck,
    Use,
    DropIt,
}
#[derive(Component, Reflect, Default, Deref, Clone, Copy)]
pub struct CardPlace(pub usize);

#[derive(Component, Reflect, Default, Deref, Clone, Copy)]
pub struct CardNumber(pub usize);

#[derive(Component, Reflect, Default, Clone, Copy)]
#[require(GameObject, Pickable, ActionToPerform, StateScoped::<GameTurnSteps>(GameTurnSteps::PerformAction))]
pub struct DraggableCard {
    pub can_afford: bool,
}

#[derive(Component, Reflect, Default, Clone, Copy)]
#[require(GameObject)]
pub struct CurrentlyDragged;

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
            .register_type::<CardPlace>()
            .register_type::<DraggableCard>()
            .register_type::<CurrentlyDragged>()
            .register_type::<CardDropZone>()
            .register_type::<ActionToPerform>()
            .register_type::<CardNumber>()
            .add_systems(
                Update,
                update_card_places.run_if(window_changed_or_component_added::<CardPlace>),
            )
            .add_systems(
                Update,
                (cards_input_system, sort_cards, update_card_color)
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                OnEnter(GameTurnSteps::PerformAction),
                despawn_recursive_by_component::<CardDisplay>,
            )
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
            .insert(CardPlace(i));
    }

    commands
        .spawn((
            Transform::from_xyz(0.0, 100.0, 0.0),
            Sprite {
                color: Color::WHITE.with_alpha(0.3),
                custom_size: Some(Vec2::new(900.0, 300.0)),
                ..default()
            },
            CardDropZone::Use,
        ))
        .observe(on_drag_over_card)
        .observe(on_drag_leave_card);

    commands
        .spawn((
            Sprite {
                color: tailwind::RED_300.with_alpha(0.7).into(),
                custom_size: Some(Vec2::new(300.0, 300.0)),
                ..default()
            },
            Transform::from_xyz(650.0, -350.0, 0.0),
            CardDropZone::Discard,
        ))
        .observe(on_drag_over_card)
        .observe(on_drag_leave_card);
}

fn update_card_color(
    mut q: Query<
        (&mut Sprite, &ActionToPerform, &DraggableCard),
        Or<(Added<ActionToPerform>, Changed<ActionToPerform>)>,
    >,
) {
    for (mut sprite, action, draggable) in q.iter_mut() {
        sprite.color = match (action, draggable.can_afford) {
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
    mut qq: Query<(&mut ActionToPerform, &DraggableCard)>,
) -> Result {
    let zone_type = q.get(trigger.target())?;
    info!("{zone_type:?}");
    let (mut action, card) = qq.get_mut(trigger.dragged)?;
    let new_action = match zone_type {
        CardDropZone::Use if card.can_afford => ActionToPerform::Use,
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
) -> Result {
    let Ok(hand) = deck_q.single() else {
        return Ok(());
    };
    let window = windows.single()?;
    let y_pos = -window.height() + inline_tweak::tweak!(-450.0);

    for card_info in hand.card_info_array().iter() {
        let offset = (inline_tweak::tweak!(200) * card_info.index) as f32;
        commands.spawn((
            Sprite {
                color: card_info.color(),
                image: asset_server.load(card_info.image_path()),
                ..default()
            },
            Transform::from_xyz(-350.0 + offset, y_pos - 300.0, card_info.index as f32 + 1.0)
                .with_scale(CARD_SIZE),
            Name::new(format!("Card Nr {}", card_info.index)),
            DraggableCard {
                can_afford: card_info.can_afford,
            },
            CardNumber(card_info.index),
            CardDisplay(card_info.card.clone()),
        ));
    }
    Ok(())
}

fn start_drag(
    trigger: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    mut help_query: Query<&mut Node, With<HelpDisplay>>,
) {
    for mut s in help_query.iter_mut() {
        s.display = Display::None;
    }
    commands
        .entity(trigger.target())
        .insert(ActionToPerform::Nothing)
        .insert(CurrentlyDragged)
        .insert(Pickable {
            is_hoverable: true,
            should_block_lower: false,
        });
}

fn sort_cards(
    cards_q: Query<(&Transform, &CardNumber, Entity), With<CardDisplay>>,
    mut commands: Commands,
) {
    let mut cards = cards_q.iter().collect::<Vec<_>>();
    cards.sort_by(|a, b| a.0.translation.x.partial_cmp(&b.0.translation.x).unwrap());
    for (i, (_, nr, entity)) in cards.iter().enumerate() {
        if nr.0 != i {
            commands.entity(*entity).insert(CardNumber(i));
        }
    }
}
fn update_card_places(windows: Query<&Window>, mut query2: Query<(&mut Transform, &CardPlace)>) {
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
) {
    let Ok((card_display, action)) = cards_q.get(trigger.target()) else {
        return;
    };
    commands
        .entity(trigger.target())
        .remove::<CurrentlyDragged>()
        .insert(Pickable {
            is_hoverable: true,
            should_block_lower: true,
        });
    let Ok(player_entity) = q.single() else {
        return;
    };
    let action_taken = match action {
        ActionToPerform::Use => ActionTaken::UseCard {
            card: (**card_display).clone(),
        },
        ActionToPerform::Discard => ActionTaken::DropCard {
            card: (**card_display).clone(),
        },
        ActionToPerform::Nothing => return,
    };
    commands.entity(player_entity).insert(action_taken);
}

fn cards_input_system(
    mut cards_q: Query<
        (&mut Transform, Option<&CurrentlyDragged>, &CardNumber),
        Without<CardPlace>,
    >,
    time: Res<Time>,
    places_q: Query<(&Transform, &CardPlace), With<CardPlace>>,
) {
    let speed = inline_tweak::tweak!(20.0);
    let delta = (time.delta_secs() * speed).min(1.0);

    for (mut t, dragged, num) in cards_q.iter_mut() {
        let dragged = dragged.is_some();
        let target_scale = if dragged {
            inline_tweak::tweak!(1.1) * CARD_SIZE
        } else {
            CARD_SIZE
        };
        t.scale = t.scale.lerp(target_scale, delta);
        if dragged {
            continue;
        }
        for (place_t, place_nr) in places_q.iter() {
            if **num != **place_nr {
                continue;
            }
            t.translation = t.translation.lerp(place_t.translation, delta);
            t.rotation = t.rotation.lerp(place_t.rotation, delta);
        }
    }
}
