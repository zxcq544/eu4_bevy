use crate::plugins::initial_boot_step::resources::initial_booting_background_screen::InitialBootingBackgroundScreen;
use bevy::prelude::*;

pub fn load_background_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_filename = "gfx/loadingscreens/fixed/load_0.dds";
    let background_image = asset_server.load(background_filename);
    commands.insert_resource(InitialBootingBackgroundScreen {
        image: background_image,
    });
}
