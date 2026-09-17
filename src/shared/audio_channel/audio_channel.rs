use bevy::prelude::*;

use crate::shared::settings::settings::Settings;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum AudioChannelUIMarker {
    Master,
    Music,
    Sfx,
}

impl AudioChannelUIMarker {
    pub fn get_volume_from_settings(self, s: &Settings) -> f32 {
        match self {
            Self::Master => s.volume_settings.get_master_volume(),
            Self::Music => s.volume_settings.get_music_volume(),
            Self::Sfx => s.volume_settings.get_sfx_volume(),
        }
    }
    pub fn set_volume_in_settings(self, s: &mut Settings, v: f32) {
        match self {
            Self::Master => s.volume_settings.set_master_volume(v),
            Self::Music => s.volume_settings.set_music_volume(v),
            Self::Sfx => s.volume_settings.set_sfx_volume(v),
        }
    }
}
