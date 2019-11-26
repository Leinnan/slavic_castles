use crate::card::Card;
use crate::resource::ResourceType;
use quicksilver::{
    lifecycle::{run, Asset, Event, Settings, State, Window},
    sound::Sound,
    Result,
};

pub struct CardSounds {
    tools_sound: Asset<Sound>,
    soldiers_sound: Asset<Sound>,
    magic_sound: Asset<Sound>,
}

impl CardSounds {
    pub fn new() -> Self {
        CardSounds {
            tools_sound: Asset::new(Sound::load("snd_card_tools.wav")),
            soldiers_sound: Asset::new(Sound::load("snd_card_soldiers.wav")),
            magic_sound: Asset::new(Sound::load("snd_card_magic.wav")),
        }
    }

    pub fn play_card_sound(&mut self, resource_type: ResourceType) {
        match resource_type {
            ResourceType::Magic => self.magic_sound.execute(|s| {
                s.play()?;
                Ok(())
            }),
            ResourceType::Soldiers => self.soldiers_sound.execute(|s| {
                s.play()?;
                Ok(())
            }),
            ResourceType::Tools => self.tools_sound.execute(|s| {
                s.play()?;
                Ok(())
            }),
        };
    }
}
