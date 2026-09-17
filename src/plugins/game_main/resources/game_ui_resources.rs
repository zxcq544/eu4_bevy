use bevy::prelude::*;

#[derive(Resource)]
pub struct GameUiResources {
    pub ui_slider_thumb_image_blue: Handle<Image>,
    pub ui_slider_thumb_image_red: Handle<Image>,
    pub ui_options_slider_background_image: Handle<Image>,
    pub ui_options_slider_left_arrow_image: Handle<Image>,
    pub ui_options_slider_right_arrow_image: Handle<Image>,
    // pub ui_slider_track_image: Handle<Image>,
    pub button_normal_image: Handle<Image>,
    pub button_wide_image: Handle<Image>,
    pub button_small_image: Handle<Image>,

    pub country_shield_frame: Handle<Image>,
    pub country_shield_frame_green_glow: Handle<Image>,
    pub country_choice_bottom_middle_block_background_image: Handle<Image>,
}
