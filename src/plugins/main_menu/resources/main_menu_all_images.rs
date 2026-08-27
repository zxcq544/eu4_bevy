use bevy::prelude::*;
#[derive(Resource)]
pub struct MainMenuAllImages {
    pub main_menu_background_image: Handle<Image>,
    pub continue_background_image: Handle<Image>,
    pub single_player_button_image: Handle<Image>,
    pub multiplayer_button_image: Handle<Image>,
    pub continue_button_image: Handle<Image>,
    pub bg_image_lower_panel_main_menu_left_button: Handle<Image>,
    pub bg_image_lower_panel_main_menu_center_button: Handle<Image>,
    pub bg_image_lower_panel_main_menu_right_button: Handle<Image>,
}
