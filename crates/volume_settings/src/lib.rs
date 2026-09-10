use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VolumeSettings {
    master_volume: f32,
    music_volume: f32,
    sound_effects_volume: f32,
}

impl VolumeSettings {
    pub fn new(master_volume: f32, music_volume: f32, sound_effects_volume: f32) -> Self {
        VolumeSettings {
            master_volume,
            music_volume,
            sound_effects_volume,
        }
    }
    pub fn get_music_volume(&self) -> f32 {
        self.master_volume * self.music_volume
    }
    pub fn get_sound_effects_volume(&self) -> f32 {
        self.master_volume * self.sound_effects_volume
    }
    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }
    pub fn get_master_volume_percentage(&self) -> f32 {
        (self.master_volume * 100.0).round()
    }
    pub fn get_music_volume_percentage(&self) -> f32 {
        (self.master_volume * self.music_volume * 100.0).round()
    }
    pub fn get_sound_effects_volume_percentage(&self) -> f32 {
        (self.master_volume * self.sound_effects_volume * 100.0).round()
    }
    pub fn set_master_volume(&mut self, value: f32) {
        self.master_volume = value;
    }
    pub fn set_music_volume(&mut self, value: f32) {
        self.music_volume = value;
    }
    pub fn set_sound_effects_volume(&mut self, value: f32) {
        self.sound_effects_volume = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume_settings() {
        let result = VolumeSettings::new(1.0, 0.52, 0.526);
        assert_eq!(result.get_music_volume(), 0.52);
        assert_eq!(result.get_sound_effects_volume(), 0.526);
        assert_eq!(result.get_master_volume(), 1.0);
        assert_eq!(result.get_master_volume_percentage(), 100.0);
        assert_eq!(result.get_music_volume_percentage(), 52.0);
        assert_eq!(result.get_sound_effects_volume_percentage(), 53.0);
    }
}
