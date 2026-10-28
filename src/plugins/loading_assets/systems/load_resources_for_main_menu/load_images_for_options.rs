use crate::plugins::options::resources::options_images::OptionsImages;
use bevy::prelude::*;

pub fn load_images_for_options(asset_server: Res<AssetServer>, mut commands: Commands) {
    info!("Loading options images");
    let settings_bg_image = asset_server.load("gfx/interface/settings/setting_bg.dds");
    let tab_in_settings_image = asset_server.load("gfx/interface/settings/fixed/tab_setting_large.dds");
    // let audio_bg_image = asset_server.load("gfx/interface/settings/audio_settings_bg.dds");
    // let controls_bg_image = asset_server.load("gfx/interface/settings/controls_settings_bg.dds");
    // let game_bg_image = asset_server.load("gfx/interface/settings/game_settings_bg.dds");
    // let video_bg_image = asset_server.load("gfx/interface/settings/video_settings_bg.dds");
    // let multiplayer_bg_image =
    //     asset_server.load("gfx/interface/settings/multiplayer_settings_bg.dds");
    let apply_and_back_button_image = asset_server.load("gfx/interface/standard_button_105.dds");
    commands.insert_resource(OptionsImages {
        settings_bg_image,
        tab_in_settings_image,
        // audio_bg_image: audio_bg_image,
        // controls_bg_image: controls_bg_image,
        // game_bg_image,
        // video_bg_image: video_bg_image,
        // multiplayer_bg_image: multiplayer_bg_image,
        apply_and_back_button_image,
    });
}
