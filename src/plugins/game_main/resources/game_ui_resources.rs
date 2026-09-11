use bevy::prelude::*;

#[derive(Resource)]
pub struct GameUiResources {
    pub ui_slider_thumb_image_blue: Handle<Image>,
    pub ui_slider_thumb_image_red: Handle<Image>,
    pub ui_slider_track_image: Handle<Image>,
}
