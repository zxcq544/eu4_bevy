use bevy::prelude::*;

#[derive(Resource)]
pub struct ButtonClickSoundEffects {
    pub button_click_general: Handle<AudioSource>,
    pub button_click_ok: Handle<AudioSource>,
    pub button_click_back: Handle<AudioSource>,
}
