use crate::plugins::game_main::resources::game_ui_resources::GameUiResources;
use bevy::prelude::*;

pub fn load_images_for_game_ui(asset_server: Res<AssetServer>, mut commands: Commands) {
    info!("Loading game ui images");
    // There are two images for thumb - red and blue scroll_drager_blue.dds and scroll_drager.dds
    let ui_slider_thumb_image_blue = asset_server.load("gfx/interface/scroll_drager_blue.dds");
    let ui_slider_thumb_image_red = asset_server.load("gfx/interface/scroll_drager.dds");
    let ui_options_slider_background_image =
        asset_server.load("gfx/interface/settings_slider_bg.dds");
    let ui_options_slider_left_arrow_image =
        asset_server.load("gfx/interface/scrollbar_leftbutton.dds");
    let ui_options_slider_right_arrow_image =
        asset_server.load("gfx/interface/scrollbar_rightbutton.dds");
    // let ui_slider_track_image = asset_server.load("gfx/interface/scroll_track.dds");

    let button_normal_image = asset_server.load("gfx/interface/button_type_1.dds");
    let button_wide_image = asset_server.load("gfx/interface/button_base_button.dds");
    let button_small_image = asset_server.load("gfx/interface/button_type_6.dds");

    let country_shield_frame = asset_server.load("gfx/interface/shield_frame.dds");
    let country_shield_frame_green_glow = asset_server.load("gfx/interface/shield_thin_glow.dds");
    let country_choice_bottom_middle_block_background_image =
        asset_server.load("gfx/interface/lobby_chat_bg.dds");
    commands.insert_resource(GameUiResources {
        ui_slider_thumb_image_blue,
        ui_slider_thumb_image_red,
        ui_options_slider_background_image,
        ui_options_slider_left_arrow_image,
        ui_options_slider_right_arrow_image,
        // ui_slider_track_image,
        button_normal_image,
        button_wide_image,
        button_small_image,
        country_shield_frame,
        country_shield_frame_green_glow,
        country_choice_bottom_middle_block_background_image,
    });
}
