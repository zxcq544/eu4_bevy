use crate::plugins::loading_assets::loading_assets::MainLoadingStepBackgroundImage;
use crate::plugins::loading_assets::resources::loading_screen_tooltip_image::LoadingScreenTooltipImage;
use bevy::prelude::*;

pub fn load_main_loading_step_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_background(&mut commands, &asset_server);
    load_tooltip(&mut commands, &asset_server);
}

fn load_tooltip(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    info!("Loading loading screen tooltip");
    let tooltip = asset_server.load("gfx/interface/Loadingscreen_loadingtip.dds");
    commands.insert_resource(LoadingScreenTooltipImage { image: tooltip });
}

fn load_background(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    info!("Loading main loading step background");
    let main_loading_step_background_image = pick_random_background_image(&asset_server);
    commands.insert_resource(MainLoadingStepBackgroundImage {
        image: main_loading_step_background_image,
    });
}

fn pick_random_background_image(asset_server: &Res<AssetServer>) -> Handle<Image> {
    let n: i32 = rand::random_range(1..=36);
    let background_filename = format!("gfx/loadingscreens/fixed/load_{}.dds", n);
    info!(
        "Loading background image from path {:?}",
        background_filename
    );
    return asset_server.load(&background_filename);
}
