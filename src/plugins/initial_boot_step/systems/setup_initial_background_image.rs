use bevy::{image::ImageLoaderSettings, prelude::*, render::render_resource::TextureFormat};
// use bevy::render::texture::TextureFormat;

use crate::{BootBackground, InitialBootingBackgroundScreen};

#[derive(Component)]
struct BootScreenEntity;

pub fn setup_initial_background_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d::default(), BootScreenEntity));
    let initial_booting_screen_handle = load_initial_background_image(asset_server);
    commands.spawn((
        Sprite {
            image: initial_booting_screen_handle.image,
            ..default()
        },
        BootScreenEntity,
        BootBackground,
    ));
}

fn load_initial_background_image(asset_server: Res<AssetServer>) -> InitialBootingBackgroundScreen {
    let image = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| {
            // Converts interpretation from sRGB to standard Linear format
            settings.is_srgb = true;
            settings.texture_format = Some(TextureFormat::Bgra8UnormSrgb);
        })
        .load("gfx/loadingscreens/load_0.dds");
    InitialBootingBackgroundScreen { image }
}
