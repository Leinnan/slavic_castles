use bevy::prelude::*;

#[derive(Default, Debug, Reflect, Component)]
pub struct CurrentActorToken;

#[derive(Deref, DerefMut, Component, Default, Reflect)]
pub struct ActorTurn(pub usize);

#[derive(Debug, Hash, PartialEq, Eq, Default, Clone, States)]
pub enum GameTurnSteps {
    #[default]
    SearchForAgents,
    ActionSelection,
    PerformAction,
}

pub fn register_system(app: &mut App) {
    app.register_type::<ActorTurn>()
        .register_type::<CurrentActorToken>()
        .init_state::<GameTurnSteps>()
        .add_systems(OnExit(GameTurnSteps::PerformAction), remove_token)
        .add_systems(
            Update,
            search_for_actors.run_if(in_state(GameTurnSteps::SearchForAgents)),
        );
}

pub fn search_for_actors(
    mut commands: Commands,
    q: Query<(&ActorTurn, Entity)>,
    mut next_state: ResMut<NextState<GameTurnSteps>>,
) {
    if q.is_empty() {
        return;
    }

    let mut lowest_delay = (usize::MAX, Entity::PLACEHOLDER);
    for (delay, entity) in q.iter() {
        if **delay < lowest_delay.0 {
            lowest_delay.0 = **delay;
            lowest_delay.1 = entity;
        }
    }
    if lowest_delay.0 < usize::MAX {
        commands.entity(lowest_delay.1).insert(CurrentActorToken);
        next_state.set(GameTurnSteps::ActionSelection);
        info!("FOUNDED ACTOR ENTITY {:?}", lowest_delay.1);
    }
}

fn remove_token(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ActorTurn), With<CurrentActorToken>>,
) {
    let Ok((entity, mut delay)) = query.single_mut() else {
        return;
    };
    delay.0 = **delay + 2;
    commands.entity(entity).remove::<CurrentActorToken>();
}
