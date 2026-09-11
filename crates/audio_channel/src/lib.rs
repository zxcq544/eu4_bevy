use bevy::prelude::*;
use settings::Settings;

#[derive(Component, Clone, Copy, Debug)]
pub enum AudioChannel {
    Master,
    Music,
    Sfx,
}

impl AudioChannel {
    pub fn get(self, s: &Settings) -> f32 {
        match self {
            Self::Master => s.volume_settings.get_master_volume(),
            Self::Music => s.volume_settings.get_music_volume(),
            Self::Sfx => s.volume_settings.get_sfx_volume(),
        }
    }
    pub fn set(self, s: &mut Settings, v: f32) {
        match self {
            Self::Master => s.volume_settings.set_master_volume(v),
            Self::Music => s.volume_settings.set_music_volume(v),
            Self::Sfx => s.volume_settings.set_sfx_volume(v),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
