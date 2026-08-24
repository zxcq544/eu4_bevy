use crate::plugins::main_menu::resources::background_image_of_main_menu::BackgroundImageOfMainMenu;
use bevy::prelude::*;

pub fn load_main_menu_background_image(asset_server: Res<AssetServer>, mut commands: Commands) {
    info!("Loading main menu background image");
    let main_menu_background_image = asset_server.load("gfx/interface/frontend_bottom_small_bg.dds");
    commands.insert_resource(BackgroundImageOfMainMenu {
        image: main_menu_background_image,
    });
}
