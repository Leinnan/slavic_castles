use core::panic;

use super::game_states::GameState;
use crate::base_systems::turn_based::{ActorTurn, CurrentActorToken, GameTurnSteps};
use crate::data::card::Card;
use crate::data::deck::{DeckAsset, HandCards};
use crate::data::player::Player;
use crate::data::player_resources::PlayerResources;
use crate::helpers::{despawn_recursive_by_component, AudioSpawnCommandExt};

use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::utils::HashMap;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use super::consts;

#[derive(Component, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Debug, Clone, Reflect)]
pub enum PlayerNumber {
    First,
    Second,
}

#[derive(Component)]
pub struct HumanPlayer;

pub struct GamePlugin;

#[derive(Component)]
pub struct HelpDisplay;

#[derive(Component)]
pub struct DeckNode;

#[derive(Resource, Reflect, Default)]
pub struct SelectedCard {
    pub display_entity: Option<Entity>,
    pub data: Option<Card>,
}

#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
pub struct GameObject;

#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
pub struct BackgroundSprite;

#[derive(serde::Deserialize, bevy::asset::Asset, Deref, DerefMut, Reflect)]
pub struct NamesAsset(pub Vec<String>);

impl NamesAsset {
    pub fn get_random(&self) -> String {
        let mut rng = thread_rng();
        let i: usize = rng.gen::<usize>() % self.0.len();
        self.0[i].clone()
    }
}

#[derive(Component, Debug, Clone)]
pub enum ActionTaken {
    UseCard { card: Card },
    DropCard { card: Card },
}

#[derive(Resource, Debug, Default, Reflect)]
pub struct TimeSinceTurnStarted(pub Stopwatch);

#[derive(Component, Debug, Default, Reflect)]
struct ExitGameTimer(pub Timer);

#[derive(Resource, Debug, Default, Reflect, Deref)]
#[reflect(Resource)]
pub struct OpponentInformation(pub PlayerInformation);

#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct PlayerInformation {
    pub name: String,
    pub start_stats: Player,
    pub deck: DeckAsset,
    pub avatar_id: i32,
}

#[derive(Component, Debug, Default, Reflect, Deref)]
pub struct AvatarId(pub i32);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (setup_music, init_players, setup_ui).chain(),
        )
        .add_systems(
            Update,
            perform_action.run_if(in_state(GameTurnSteps::PerformAction)),
        )
        .add_systems(
            OnEnter(GameTurnSteps::ActionSelection),
            switch_player.run_if(not(game_ended_condition)),
        )
        .add_systems(
            OnEnter(GameTurnSteps::SearchForAgents),
            (end_game).run_if(game_ended_condition),
        )
        .add_systems(
            Update,
            (handle_card_events, ai_select_card).run_if(in_state(GameTurnSteps::ActionSelection)),
        )
        .add_systems(
            Update,
            (
                update_ui,
                update_deck_visibility,
                update_timers,
                card_sounds,
            )
                .run_if(in_state(GameState::Game)),
        )
        .init_resource::<SelectedCard>()
        .init_resource::<TimeSinceTurnStarted>()
        .register_type::<GameObject>()
        .register_type::<HandCards>()
        .register_type::<ExitGameTimer>()
        .register_type::<OpponentInformation>()
        .register_type::<PlayerInformation>()
        .register_type::<TimeSinceTurnStarted>()
        .add_systems(
            OnExit(GameState::Game),
            (despawn_recursive_by_component::<GameObject>,),
        )
        .add_systems(
            Update,
            esc_to_menu
                .run_if(not(game_ended_condition))
                .run_if(in_state(GameState::Game)),
        );
    }
}

fn end_game(query: Query<&Player, With<HumanPlayer>>, mut commands: Commands) {
    let Ok(player) = query.get_single() else {
        panic!("SDD");
    };
    let player_won = player.has_max_possible_tower() || player.is_alive();
    info!("PLAYER WON? {}", player_won);
    let sound = if player_won {
        "snd/game_won.ogg"
    } else {
        "snd/game_lost.ogg"
    };
    commands
        .spawn(ExitGameTimer(Timer::from_seconds(4.0, TimerMode::Once)))
        .insert(GameObject)
        .insert(Name::new("GAME END TIMER"));
    commands.play_sound(sound);
}

fn update_timers(
    mut timer: ResMut<TimeSinceTurnStarted>,
    mut exit_game_timer: Query<&mut ExitGameTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());
    for mut t in exit_game_timer.iter_mut() {
        t.0.tick(time.delta());
        if t.0.finished() {
            next_state.set(GameState::Menu);
            return;
        }
    }
}

pub fn switch_player(
    mut q: Query<(&Name, &mut PlayerResources), With<CurrentActorToken>>,
    mut timer: ResMut<TimeSinceTurnStarted>,
) {
    let (player, mut resources) = q.single_mut();
    info!("Switch player: {}", player);
    resources.update_resources();
    timer.0.reset();
}

fn update_deck_visibility(
    q: Query<Entity, With<DeckNode>>,
    mut commands: Commands,
    cur_player_q: Query<Option<&HumanPlayer>, With<CurrentActorToken>>,
    state: Res<State<GameTurnSteps>>,
    mut selected_card: ResMut<SelectedCard>,
) {
    if !state.is_changed() {
        return;
    }
    let Ok(cur_player_human) = cur_player_q.get_single() else {
        return;
    };
    let Ok(e) = q.get_single() else {
        return;
    };
    if cur_player_human.is_none() && *state == GameTurnSteps::ActionSelection {
        if let Some(mut cmd_e) = commands.get_entity(e) {
            selected_card.data = None;
            selected_card.display_entity = None;
            cmd_e.insert(Visibility::Hidden);
            // cmd_e.despawn_recursive();
        }
    }
}

fn update_ui(
    player_query: Query<(&Player, &PlayerNumber, &PlayerResources, &Name)>,
    mut ui: Query<(&mut Text, &PlayerNumber)>,
) {
    let mut player_texts = HashMap::new();
    for (player, player_num, resources, name) in &player_query {
        player_texts.insert(
            *player_num,
            (
                name.as_str().to_owned(),
                format!(
                    "\nTower: {0}\nWalls: {1}\n{2}",
                    player.tower_hp,
                    player.walls_hp,
                    resources.print()
                ),
            ),
        );
    }
    for (mut text, player_num) in &mut ui {
        if let Some(player_description) = player_texts.remove(player_num) {
            text.sections[1].value = player_description.1;
            text.sections[0].value = player_description.0;
        }
    }
}

pub fn init_players(
    mut commands: Commands,
    player: Res<PlayerInformation>,
    opponent: Res<OpponentInformation>,
    mut next_state: ResMut<NextState<GameTurnSteps>>,
) {
    commands
        .spawn((
            Name::new(player.name.clone()),
            player.start_stats,
            PlayerNumber::First,
            HumanPlayer,
            ActorTurn(0),
            AvatarId(player.avatar_id),
            PlayerResources::default(),
            HandCards::generate_random(&player.deck.0),
        ))
        .insert(GameObject);
    commands
        .spawn((
            Name::new(opponent.name.clone()),
            opponent.start_stats,
            ActorTurn(1),
            PlayerNumber::Second,
            AvatarId(opponent.avatar_id),
            PlayerResources::default(),
            HandCards::generate_random(&opponent.deck.0),
        ))
        .insert(GameObject);
    next_state.set(GameTurnSteps::SearchForAgents);
}

fn setup_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("snd/start_game.ogg"),
        ..default()
    });
}

fn card_sounds(mut commands: Commands, q: Query<&ActionTaken, Added<ActionTaken>>) {
    for action in q.iter() {
        let sound = match action {
            ActionTaken::UseCard { card } => card.get_sound_asset(),
            ActionTaken::DropCard { card: _ } => "snd/card_dismiss.ogg".to_owned(),
        };
        commands.play_sound(sound);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let header_style = TextStyle {
        font: asset_server.load(consts::LABEL_FONT),
        font_size: 45.0,
        color: Color::GOLD,
    };
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("img/ingame_bg.png"),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..Default::default()
        })
        .insert(Name::new("BG"))
        .insert(GameObject)
        .insert(BackgroundSprite);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::new(Val::Auto, Val::Auto, Val::Px(30.0), Val::Auto),
                    padding: UiRect::all(Val::Px(15.0)),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::hex("2c422e").unwrap().into(),
                ..Default::default()
            },
            Name::new("Help display"),
            HelpDisplay,
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section("Help", header_style.clone()));
            p.spawn(
                TextBundle::from_section(
                    consts::HELP_TEXT,
                    TextStyle {
                        font: asset_server.load(consts::REGULAR_FONT),
                        font_size: 25.0,
                        color: Color::hex("fcfd9e").unwrap(),
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
        });
}

fn esc_to_menu(
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut Style, With<HelpDisplay>>,
) {
    if keys.just_released(KeyCode::Escape) {
        next_state.set(GameState::Menu);
        keys.reset(KeyCode::Escape);
    } else if keys.just_released(KeyCode::KeyH) {
        for mut style in &mut query {
            style.display = if style.display == Display::Flex {
                Display::None
            } else {
                Display::Flex
            };
        }
        keys.reset(KeyCode::KeyH);
    }
}

fn handle_card_events(
    any_action: Query<&ActionTaken>,
    mut next_state: ResMut<NextState<GameTurnSteps>>,
) {
    let Ok(action_to_do) = any_action.get_single() else {
        return;
    };
    info!("{:#?}", action_to_do);
    next_state.set(GameTurnSteps::PerformAction);
}

fn ai_select_card(
    mut commands: Commands,
    cur_player_q: Query<(&CurrentActorToken, &HandCards, Entity), Without<HumanPlayer>>,
    time_since: Res<TimeSinceTurnStarted>,
    mut random_wait_time: Local<f32>,
) {
    let Ok((_, hand, e)) = cur_player_q.get_single() else {
        return;
    };
    if *random_wait_time < 1.0 {
        *random_wait_time = 1.5; // TODO make it random
    }
    if time_since.0.elapsed_secs() < *random_wait_time {
        return;
    }
    let i = hand.rnd();
    let random_card = hand.cards[i].clone();

    commands
        .entity(e)
        .insert(ActionTaken::UseCard { card: random_card });
}

pub fn perform_action(
    any_action: Query<&ActionTaken>,
    mut players_q: Query<(
        Entity,
        &mut PlayerResources,
        &mut Player,
        &mut HandCards,
        Option<&CurrentActorToken>,
    )>,
    mut next_state: ResMut<NextState<GameTurnSteps>>,
    deck: Res<Assets<DeckAsset>>,
    mut commands: Commands,
) {
    let Ok(action_to_do) = any_action.get_single() else {
        return;
    };
    let Some(deck_asset) = deck.iter().next() else {
        panic!("NO DECK ASSET");
    };
    let cards: &Vec<Card> = deck_asset.1.as_ref();
    for (e, mut res, mut player, mut hand, is_caller) in players_q.iter_mut() {
        let is_user = is_caller.is_some();
        let card_id;
        match action_to_do {
            ActionTaken::UseCard { card } => {
                if is_caller.is_some() {
                    res.change_resource_amount(card.cost_resource, -card.cost_amount);
                }
                card_id = hand.cards.iter().position(|c| c == card);
                let res_change = card.resource_amount_change(is_user);
                res.change_resource_amount(res_change.0, res_change.1);
                let prod = card.production_change(is_user);
                res.change_resource_production(prod.0, prod.1);
                let damage = card.damage(is_user);
                player.give_damage(damage.0, damage.1);
                player.make_tower_higher(card.tower_growth(is_user));
                player.make_walls_higher(card.walls_growth(is_user));
            }
            ActionTaken::DropCard { card } => card_id = hand.cards.iter().position(|c| c == card),
        };
        if is_user {
            if let Some(id) = card_id {
                hand.replace_card(id, &res, cards);
            }
            commands.entity(e).remove::<ActionTaken>();
        }
    }
    next_state.set(GameTurnSteps::SearchForAgents);
}

fn game_ended_condition(query: Query<&Player>) -> bool {
    for player in &query {
        if !player.is_alive() || player.has_max_possible_tower() {
            return true;
        }
    }

    false
}
