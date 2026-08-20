use crate::plugins::loading_assets::loading_assets::MainLoadingStepBackgroundImage;
use crate::plugins::loading_assets::resources::loading_screen_tooltip_image::LoadingScreenTooltipImage;
use bevy::{image::ImageLoaderSettings, prelude::*, render::render_resource::TextureFormat};
use path_clean::PathClean;
use settings::Settings;
use std::{fs, path::Path};

pub fn load_main_loading_step_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
) {
    load_background(&mut commands, &asset_server, &settings);
    load_tooltip(&mut commands, &asset_server);
}

fn load_tooltip(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    info!("Loading loading screen tooltip");
    let tooltip = asset_server.load("gfx/interface/Loadingscreen_loadingtip.dds");
    commands.insert_resource(LoadingScreenTooltipImage { image: tooltip });
}

fn load_background(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    settings: &Res<Settings>,
) {
    info!("Loading main loading step background");
    let main_loading_step_background_image = pick_random_background_image(&asset_server, settings);
    commands.insert_resource(MainLoadingStepBackgroundImage {
        image: main_loading_step_background_image,
    });
}

fn pick_random_background_image(
    asset_server: &Res<AssetServer>,
    settings: &Res<Settings>,
) -> Handle<Image> {
    let n: i32 = rand::random_range(1..=36);
    let background_filename = format!("gfx/loadingscreens/load_{}.dds", n);
    let full_path = Path::new(&settings.eu4_folder)
        .join(&background_filename)
        .clean();
    let file_size_info = fs::metadata(&full_path).unwrap();
    let file_size = file_size_info.len();
    info!("Loading background image from path {:?}", full_path);
    // Compressed files are 3MB and only load like that
    if file_size == 3_145_856 {
        return asset_server.load(&background_filename);
    } else {
        // This breaks on compressed dds files which are 3 MB background images in EU4
        return asset_server
            .load_builder()
            .with_settings(|settings: &mut ImageLoaderSettings| {
                // EU4 uses BGRA instead of RGBA at least for loading screen
                settings.is_srgb = true;
                settings.texture_format = Some(TextureFormat::Bgra8UnormSrgb);
            })
            .load(&background_filename);
    }
}
