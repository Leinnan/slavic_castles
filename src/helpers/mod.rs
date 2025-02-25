pub mod wasm_resize;

use bevy::ecs::system::EntityCommands;
use bevy::ecs::world::Command;
use bevy::{asset::AssetPath, prelude::*};

/// Command for playing a standard bevy audio asset
pub struct AudioSpawnCommand<'a> {
    /// Path to the sound asset
    pub path: AssetPath<'a>,
    /// Sound playback settings
    pub settings: PlaybackSettings,
    /// Entity to attach AudioBundle, if none it will spawn new entity
    pub entity: Option<Entity>,
}

impl Command for AudioSpawnCommand<'static> {
    fn apply(self, world: &mut World) {
        let asset = world.get_resource::<AssetServer>().unwrap();
        let source = asset.load(&self.path);
        let bundle = (self.settings, AudioPlayer::new(source));
        match self.entity {
            Some(e) => {
                if let Ok(mut entity) = world.get_entity_mut(e) {
                    entity.insert(bundle);
                }
            }
            None => {
                world.spawn(bundle);
            }
        }
    }
}

/// Trait for playing sounds with commands
pub trait AudioSpawnCommandExt {
    /// Command for playing a standard bevy audio asset with default settings.
    ///
    /// Remember that if the sound asset is not already loaded, the sound will have delay before playing because it needs to load first.
    fn play_sound(&mut self, data: impl Into<AssetPath<'static>>);

    /// Command for playing a standard bevy audio asset with settings.
    ///
    /// Remember that if the sound asset is not already loaded, the sound will have delay before playing because it needs to load first.
    fn play_sound_with_settings(
        &mut self,
        asset_id: impl Into<AssetPath<'static>>,
        settings: PlaybackSettings,
    );
}

impl AudioSpawnCommandExt for Commands<'_, '_> {
    fn play_sound(&mut self, path: impl Into<AssetPath<'static>>) {
        self.queue(AudioSpawnCommand {
            path: path.into(),
            settings: Default::default(),
            entity: None,
        });
    }
    fn play_sound_with_settings(
        &mut self,
        path: impl Into<AssetPath<'static>>,
        settings: PlaybackSettings,
    ) {
        self.queue(AudioSpawnCommand {
            path: path.into().clone(),
            settings,
            entity: None,
        });
    }
}

impl AudioSpawnCommandExt for EntityCommands<'_> {
    fn play_sound(&mut self, path: impl Into<AssetPath<'static>>) {
        let entity = Some(self.id());
        self.commands().queue(AudioSpawnCommand {
            path: path.into(),
            settings: Default::default(),
            entity,
        });
    }

    fn play_sound_with_settings(
        &mut self,
        path: impl Into<AssetPath<'static>>,
        settings: PlaybackSettings,
    ) {
        let entity = Some(self.id());
        self.commands().queue(AudioSpawnCommand {
            path: path.into().clone(),
            settings,
            entity,
        });
    }
}

pub fn despawn_recursive_by_component<T: bevy::prelude::Component>(
    q: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for e in q.iter() {
        let Some(entity) = commands.get_entity(e) else {
            continue;
        };
        entity.despawn_recursive();
    }
}
