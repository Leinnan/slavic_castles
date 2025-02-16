use bevy::sprite::MaterialMesh2dBundle;
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

const CARD_SIZE: Vec3 = Vec3::new(256.0, 350.0, 1.0);

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

#[derive(Component, Reflect, Default, Deref, Clone, Copy)]
pub struct IsCardDragged(pub bool);

#[derive(Component, Debug, PartialEq, Clone, Reflect, Deref)]
pub struct CardDisplay(pub Card);

#[derive(Component, Reflect, Default, Deref, Clone, Copy)]
pub struct DragStartPos(pub Vec3);

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<CardDisplay>()
            .register_type::<CardPlace>()
            .register_type::<IsCardDragged>()
            .register_type::<DragStartPos>()
            .register_type::<CardNumber>()
            .add_systems(
                Update,
                (
                    card_drag_start,
                    update_background,
                    card_drag_end,
                    cards_input_system,
                    sort_cards,
                )
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
            );
    }
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
            .spawn(TransformBundle::from_transform(
                Transform::from_xyz(inline_tweak::tweak!(-600.0) + offset, y_pos, i as f32 + 1.0)
                    .with_rotation(Quat::from_rotation_z(min + step * i as f32)),
            ))
            .insert(Name::new(format!("Place{}", i)))
            .insert(GameObject)
            .insert(CardPlace(i));
    }
}

fn add_cards(
    windows: Query<&Window>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    deck_q: Query<(&HandCards, &PlayerResources, Option<&CurrentActorToken>), With<HumanPlayer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        let offset = (inline_tweak::tweak!(200) * i) as f32;
        let material = StandardMaterial {
            base_color: if res.can_afford_card(c) {
                Color::WHITE
            } else {
                Color::WHITE.darker(0.2)
                // Color::GRAY
            },
            base_color_texture: asset_server.load(format!("cards/{}.png", c.id)).into(),
            ..default()
        };
        let material = materials.add(material);
        commands.spawn((
            Mesh3d(meshes.add(Rectangle::default()).into()),
            MeshMaterial3d(material),
            Transform::from_xyz(-350.0 + offset, y_pos - 300.0, i as f32 + 1.0)
                .with_scale(CARD_SIZE),
            Name::new(format!("Card Nr {}", i)),
            IsCardDragged(false),
            CardNumber(i),
            CardDisplay(c.clone()),
            GameObject,
            // On::<Pointer<DragStart>>::target_insert(IsCardDragged(true)), // Disable picking
            // On::<Pointer<DragEnd>>::target_insert(IsCardDragged(false)),  // Re-enable picking
            // On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            //     transform.translation.x += drag.delta.x; // Make the square follow the mouse
            //     transform.translation.y -= drag.delta.y;
            //     transform.translation.z = 150.0;
            //     let clamped = inline_tweak::tweak!(0.35);
            //     let speed = inline_tweak::tweak!(0.05);
            //     let target_rotation = Quat::from_rotation_z(
            //         (drag.delta.x / inline_tweak::tweak!(-1.0)).clamp(-clamped, clamped),
            //     );
            //     transform.rotation = transform.rotation.lerp(target_rotation, speed);
            // }),
            // bevy_mod_picking::focus::PickingInteraction::default(),
        ));
    }
}

fn card_drag_start(
    cards_q: Query<(&Transform, Entity)>,
    mut ev: EventReader<Pointer<DragStart>>,
    mut help_query: Query<&mut Node, With<HelpDisplay>>,
    mut commands: Commands,
) {
    for e in ev.read() {
        for mut s in help_query.iter_mut() {
            s.display = Display::None;
        }
        if let Ok((t, entity)) = cards_q.get(e.target) {
            commands.entity(entity).insert(DragStartPos(t.translation));
        }
    }
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
        t.translation.y = window.height() * inline_tweak::tweak!(-0.5)
            + inline_tweak::tweak!(180.0)
            + pos_modifiers[**place];
        t.translation.x =
            inline_tweak::tweak!(-360.0) + (**place as f32) * inline_tweak::tweak!(180.0);
    }
}

fn card_drag_end(
    windows: Query<&Window>,
    cards_q: Query<(&Transform, &DragStartPos, &CardDisplay, Entity)>,
    mut ev: EventReader<Pointer<DragEnd>>,
    mut commands: Commands,
    q: Query<(Entity, &PlayerResources), With<CurrentActorToken>>,
) {
    let window = windows.single();
    let move_to_use = 0.3 * window.height();
    let move_to_drop = -0.2 * window.height();
    for e in ev.read() {
        if let Ok((t, start_pos, card_display, entity)) = cards_q.get(e.target) {
            let movement = t.translation - **start_pos;
            commands.entity(entity).remove::<DragStartPos>();
            let Ok((player_entity, player_resources)) = q.get_single() else {
                return;
            };
            if movement.y > move_to_use {
                if player_resources.can_afford_card(card_display) {
                    commands.entity(player_entity).insert(ActionTaken::UseCard {
                        card: (**card_display).clone(),
                    });
                } else {
                    info!("CANNOT CALL THIS CARD");
                }
            } else if movement.y < move_to_drop {
                commands
                    .entity(player_entity)
                    .insert(ActionTaken::DropCard {
                        card: (**card_display).clone(),
                    });
            }
        }
    }
}

fn cards_input_system(
    mut cards_q: Query<(&mut Transform, &IsCardDragged, &CardNumber), Without<CardPlace>>,
    time: Res<Time>,
    places_q: Query<(&Transform, &CardPlace), With<CardPlace>>,
) {
    let speed = inline_tweak::tweak!(20.0);
    let delta = (time.delta_secs() * speed).min(1.0);

    for (mut t, dragged, num) in cards_q.iter_mut() {
        let target_scale = if **dragged {
            inline_tweak::tweak!(1.1) * CARD_SIZE
        } else {
            CARD_SIZE
        };
        t.scale = t.scale.lerp(target_scale, delta);
        if **dragged {
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
