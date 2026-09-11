use crate::plugins::game_main::resources::game_ui_resources::GameUiResources;
use bevy::prelude::*;

pub fn load_images_for_game_ui(asset_server: Res<AssetServer>, mut commands: Commands) {
    info!("Loading game ui images");
    // There are two images for thumb - red and blue scroll_drager_blue.dds and scroll_drager.dds
    let ui_slider_thumb_image_blue = asset_server.load("gfx/interface/scroll_drager_blue.dds");
    let ui_slider_thumb_image_red = asset_server.load("gfx/interface/scroll_drager.dds");
    let ui_slider_track_image = asset_server.load("gfx/interface/scroll_track.dds");
    commands.insert_resource(GameUiResources {
        ui_slider_thumb_image_blue,
        ui_slider_thumb_image_red,
        ui_slider_track_image,
    });
}
