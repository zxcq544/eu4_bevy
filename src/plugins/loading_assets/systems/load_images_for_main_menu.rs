use bevy::prelude::*;

use crate::plugins::main_menu::resources::{
    background_image_for_continue::BackgroundImageForContinue,
    background_image_of_main_menu::BackgroundImageOfMainMenu,
};

pub fn load_images_for_main_menu(asset_server: Res<AssetServer>, mut commands: Commands) {
    load_main_menu_background_image(&asset_server, &mut commands);
    load_main_menu_continue_background_image(&asset_server, &mut commands);
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
