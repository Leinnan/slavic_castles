use bevy::color::palettes::tailwind;
use bevy::{prelude::*, reflect::Reflect};

use crate::base_systems::turn_based::{CurrentActorToken, GameTurnSteps};
use crate::data::card::Card;
use crate::data::deck::HandCards;
use crate::data::player_resources::PlayerResources;
use crate::helpers::despawn_recursive_by_component;
use crate::states::game::{
    self, ActionTaken, BackgroundSprite, GameObject, HelpDisplay, HumanPlayer,
};
use crate::states::game_states::GameState;

const CARD_SIZE: Vec3 = Vec3::new(1.0, 1.0, 1.0);

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
#[require(GameObject)]
pub struct DraggableCard;

#[derive(Component, Reflect, Default, Clone, Copy)]
#[require(GameObject)]
pub struct CurrentlyDragged;

#[derive(Component, Reflect, Default, Clone, Copy)]
#[require(GameObject, DraggableCard)]
pub struct CanUseCard;

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
                (update_background, cards_input_system, sort_cards)
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
            .add_observer(start_drag)
            .add_observer(drag)
            .add_observer(end_drag);
    }
}

fn drag(trigger: Trigger<Pointer<Drag>>, mut drag: Query<&mut Transform, With<DraggableCard>>) {
    let Ok(mut transform) = drag.get_mut(trigger.entity()) else {
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
    let window = windows.single();
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

fn on_drag_over_card(
    trigger: Trigger<Pointer<DragEnter>>,
    q: Query<&CardDropZone>,
    qq: Query<Option<&CanUseCard>>,
    mut commands: Commands,
) {
    let Ok(zone_type) = q.get(trigger.entity()) else {
        return;
    };
    info!("{zone_type:?}");
    let v = match zone_type {
        CardDropZone::Use if qq.get(trigger.dragged).is_ok_and(|e| e.is_some()) => {
            ActionToPerform::Use
        }
        CardDropZone::Discard => ActionToPerform::Discard,
        _ => ActionToPerform::Nothing,
    };
    commands.entity(trigger.dragged).insert(v);
}

fn on_drag_leave_card(
    trigger: Trigger<Pointer<DragLeave>>,
    q: Query<&CardDropZone>,
    mut commands: Commands,
) {
    let Ok(zone_type) = q.get(trigger.entity()) else {
        return;
    };
    info!("Leave {zone_type:?}");
    commands
        .entity(trigger.dragged)
        .insert(ActionToPerform::Nothing);
}

fn add_cards(
    windows: Query<&Window>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    deck_q: Query<(&HandCards, &PlayerResources, Option<&CurrentActorToken>), With<HumanPlayer>>,
) {
    info!("ADD CARDS");
    let Ok((deck, res, token)) = deck_q.get_single() else {
        return;
    };
    if token.is_none() {
        return;
    }
    let window = windows.single();
    let y_pos = -window.height() + inline_tweak::tweak!(-450.0);

    for (i, c) in deck.cards.iter().enumerate() {
        let color = if res.can_afford_card(c) {
            Color::WHITE
        } else {
            Color::WHITE.darker(0.4)
        };
        let offset = (inline_tweak::tweak!(200) * i) as f32;
        let mut e = commands.spawn((
            Sprite {
                color,
                image: asset_server.load(format!("cards/{}.png", c.id)),
                ..default()
            },
            Transform::from_xyz(-350.0 + offset, y_pos - 300.0, i as f32 + 1.0)
                .with_scale(CARD_SIZE),
            Name::new(format!("Card Nr {}", i)),
            DraggableCard,
            PickingBehavior {
                is_hoverable: true,
                should_block_lower: true,
            },
            CardNumber(i),
            CardDisplay(c.clone()),
        ));
        if res.can_afford_card(c) {
            e.insert(CanUseCard);
        }
    }
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
        .entity(trigger.entity())
        .insert(ActionToPerform::Nothing)
        .insert(CurrentlyDragged)
        .insert(PickingBehavior {
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
fn update_background(
    windows: Query<&Window>,
    mut query: Query<&mut Sprite, With<BackgroundSprite>>,
    mut query2: Query<(&mut Transform, &CardPlace)>,
) {
    let window = windows.single();
    for mut sprite in query.iter_mut() {
        sprite.custom_size = Some(Vec2::new(window.width(), window.height()));
    }
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
    let Ok((card_display, action)) = cards_q.get(trigger.entity()) else {
        return;
    };
    commands
        .entity(trigger.entity())
        .remove::<CurrentlyDragged>()
        .insert(PickingBehavior {
            is_hoverable: true,
            should_block_lower: true,
        });
    let Ok(player_entity) = q.get_single() else {
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
