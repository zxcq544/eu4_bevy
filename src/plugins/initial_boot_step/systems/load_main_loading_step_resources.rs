use crate::plugins::loading_assets::resources::loading_screen_tooltip_image::LoadingScreenTooltipImage;
use crate::plugins::loading_assets::{
    loading_assets::MainLoadingStepBackgroundImage,
    resources::loading_screen_status_image::LoadingScreenStatusImage,
};
use bevy::{image::ImageLoaderSettings, prelude::*, render::render_resource::TextureFormat};

pub fn load_main_loading_step_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_background(&mut commands, &asset_server);
    load_tooltip(&mut commands, &asset_server);
    load_status(&mut commands, &asset_server);
}

fn load_tooltip(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    info!("Loading loading screen tooltip");
    let tooltip = asset_server.load("gfx/interface/Loadingscreen_loadingtip.dds");
    commands.insert_resource(LoadingScreenTooltipImage { image: tooltip });
}

fn load_status(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    info!("Loading loading screen status");
    let status = asset_server.load("gfx/interface/Loadingscreen_loadingstatus.dds");
    commands.insert_resource(LoadingScreenStatusImage { image: status });
}

fn load_background(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    info!("Loading main loading step background");
    let main_loading_step_background_image = pick_random_background_image(&asset_server);
    // info!(
    //     "Main loading step background image: {:?}",
    //     main_loading_step_background_image
    // );
    commands.insert_resource(MainLoadingStepBackgroundImage {
        image: main_loading_step_background_image,
    });
}

fn pick_random_background_image(asset_server: &Res<AssetServer>) -> Handle<Image> {
    let n: i32 = rand::random_range(1..=36);
    let background_filename = format!("gfx/loadingscreens/load_{}.dds", n);
    asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| {
            // EU4 uses BGRA instead of RGBA at least for loading screen
            settings.is_srgb = true;
            settings.texture_format = Some(TextureFormat::Bgra8UnormSrgb);
        })
        .load(background_filename)
}
