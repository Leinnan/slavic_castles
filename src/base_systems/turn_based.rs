use crate::states::game_states::GameState;
use bevy::prelude::*;

#[derive(Default, Debug, Reflect, Component)]
pub struct CurrentActorToken;

#[derive(Deref, DerefMut, Component, Default, Reflect)]
pub struct ActorTurn(pub usize);

#[derive(Debug, Hash, PartialEq, Eq, Default, Clone, SubStates)]
#[source(GameState = GameState::Game)]
pub enum GameTurnSteps {
    #[default]
    SearchForAgents,
    ActionSelection,
    PerformAction,
}

pub fn register_system(app: &mut App) {
    app.register_type::<ActorTurn>()
        .register_type::<CurrentActorToken>()
        .add_sub_state::<GameTurnSteps>()
        .add_systems(OnExit(GameTurnSteps::PerformAction), remove_token)
        .add_systems(
            Update,
            search_for_actors.run_if(
                in_state(GameTurnSteps::SearchForAgents)
                    .and(not(crate::states::game::game_ended_condition)),
            ),
        );
}

pub fn search_for_actors(
    mut commands: Commands,
    q: Query<(&ActorTurn, Entity)>,
    mut next_state: ResMut<NextState<GameTurnSteps>>,
) -> Result {
    if q.is_empty() {
        return Ok(());
    }

    let mut lowest_delay = (usize::MAX, Entity::PLACEHOLDER);
    for (delay, entity) in q.iter() {
        if **delay < lowest_delay.0 {
            lowest_delay.0 = **delay;
            lowest_delay.1 = entity;
        }
    }
    if lowest_delay.0 < usize::MAX {
        commands
            .get_entity(lowest_delay.1)?
            .insert(CurrentActorToken);
        next_state.set(GameTurnSteps::ActionSelection);
        info!("FOUNDED ACTOR ENTITY {:?}", lowest_delay.1);
    }

    Ok(())
}

fn remove_token(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ActorTurn), With<CurrentActorToken>>,
) {
    warn!("Remove token");
    let Ok((entity, mut delay)) = query.single_mut() else {
        return;
    };
    delay.0 = **delay + 2;
    if let Ok(mut e) = commands.get_entity(entity) {
        e.remove::<CurrentActorToken>();
    }
}
