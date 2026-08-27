use crate::plugins::main_menu::resources::main_menu_all_images::MainMenuAllImages;
use bevy::prelude::*;

pub fn load_images_for_main_menu(asset_server: Res<AssetServer>, mut commands: Commands) {
    load_main_menu_all_images(&asset_server, &mut commands);
}

fn load_main_menu_all_images(asset_server: &Res<AssetServer>, commands: &mut Commands) {
    info!("Loading main menu all images");
    let main_menu_background_image =
        asset_server.load("gfx/interface/frontend_bottom_small_bg.dds");
    let continue_background_image = asset_server.load("gfx/interface/frontend_continue_bg.dds");
    let single_player_button_image = asset_server.load("gfx/interface/frontend_sp_button.dds");
    let multiplayer_button_image = asset_server.load("gfx/interface/frontend_mp_button.dds");
    let bg_image_lower_panel_main_menu_left_button =
        asset_server.load("gfx/interface/frontend_panel_button_left.dds");
    let bg_image_lower_panel_main_menu_center_button =
        asset_server.load("gfx/interface/frontend_panel_button_center.dds");
    let bg_image_lower_panel_main_menu_right_button =
        asset_server.load("gfx/interface/frontend_panel_button_right.dds");
    commands.insert_resource(MainMenuAllImages {
        main_menu_background_image,
        continue_background_image,
        single_player_button_image,
        multiplayer_button_image,
        bg_image_lower_panel_main_menu_left_button,
        bg_image_lower_panel_main_menu_center_button,
        bg_image_lower_panel_main_menu_right_button,
    });
}
