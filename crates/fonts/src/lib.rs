use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// This one is used in settings.json
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Fonts {
    pub loading_screen_tooltip_font: String,
    pub loading_screen_loading_text_font: String,
    pub main_font: String,
    pub button_font: String,
}

// This one is used in bevy to load Handles of fonts
#[derive(Resource, Debug, Clone)]
pub struct FontHandles {
    pub loading_screen_tooltip_font: Handle<Font>,
    pub loading_screen_loading_text_font: Handle<Font>,
    pub main_font: Handle<Font>,
    pub button_font: Handle<Font>,
}
