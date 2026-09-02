use bevy::prelude::*;

#[derive(Resource)]
pub struct OptionsImages {
    pub settings_bg_image: Handle<Image>,
    pub audio_bg_image: Handle<Image>,
    pub controls_bg_image: Handle<Image>,
    pub game_bg_image: Handle<Image>,
    pub video_bg_image: Handle<Image>,
    pub multiplayer_bg_image: Handle<Image>,
    pub apply_and_back_button_image: Handle<Image>,
}
