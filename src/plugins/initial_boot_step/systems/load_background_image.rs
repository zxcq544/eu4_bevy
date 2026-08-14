use crate::plugins::initial_boot_step::resources::initial_booting_background_screen::InitialBootingBackgroundScreen;
use bevy::{image::ImageLoaderSettings, prelude::*, render::render_resource::TextureFormat};

pub fn load_background_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_filename = "gfx/loadingscreens/load_0.dds";
    let background_image = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| {
            // EU4 uses BGRA instead of RGBA at least for loading screen
            settings.is_srgb = true;
            settings.texture_format = Some(TextureFormat::Bgra8UnormSrgb);
        })
        .load(background_filename);
    commands.insert_resource(InitialBootingBackgroundScreen {
        image: background_image,
    });
}
