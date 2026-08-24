use crate::plugins::main_menu::resources::{
    background_image_for_continue::BackgroundImageForContinue,
    background_image_of_main_menu::BackgroundImageOfMainMenu,
    main_menu_multiplayer_button_image::MainMenuMultiplayerButtonImage,
    main_menu_single_player_button_image::MainMenuSinglePlayerButtonImage,
};
use bevy::prelude::*;

pub fn load_images_for_main_menu(asset_server: Res<AssetServer>, mut commands: Commands) {
    load_main_menu_background_image(&asset_server, &mut commands);
    load_main_menu_continue_background_image(&asset_server, &mut commands);
    load_main_menu_single_player_button_image(&asset_server, &mut commands);
    load_main_menu_multiplayer_button_image(&asset_server, &mut commands);
}

fn load_main_menu_background_image(asset_server: &Res<AssetServer>, commands: &mut Commands) {
    info!("Loading main menu background image");
    let main_menu_background_image =
        asset_server.load("gfx/interface/frontend_bottom_small_bg.dds");
    commands.insert_resource(BackgroundImageOfMainMenu {
        image: main_menu_background_image,
    });
}

fn load_main_menu_continue_background_image(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
) {
    info!("Loading main menu continue background image");
    let main_menu_continue_background_image =
        asset_server.load("gfx/interface/frontend_continue_bg.dds");
    commands.insert_resource(BackgroundImageForContinue {
        image: main_menu_continue_background_image,
    });
}

fn load_main_menu_single_player_button_image(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
) {
    info!("Loading main menu single player button image");
    let main_menu_single_player_button_image =
        asset_server.load("gfx/interface/frontend_sp_button.dds");
    commands.insert_resource(MainMenuSinglePlayerButtonImage {
        image: main_menu_single_player_button_image,
    });
}

fn load_main_menu_multiplayer_button_image(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
) {
    info!("Loading main menu multiplayer button image");
    let main_menu_multiplayer_button_image =
        asset_server.load("gfx/interface/frontend_mp_button.dds");
    commands.insert_resource(MainMenuMultiplayerButtonImage {
        image: main_menu_multiplayer_button_image,
    });
}
