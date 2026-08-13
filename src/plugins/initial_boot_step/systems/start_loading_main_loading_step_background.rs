use crate::plugins::loading_assets::loading_assets::MainLoadingStepBackgroundImage;
use bevy::prelude::*;

pub fn start_loading_main_loading_step_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("Loading main loading step background");
    let main_loading_step_background_image = pick_random_background_image(asset_server);
    info!(
        "Main loading step background image: {:?}",
        main_loading_step_background_image
    );
    commands.insert_resource(MainLoadingStepBackgroundImage {
        image: main_loading_step_background_image,
    });
}

fn pick_random_background_image(asset_server: Res<AssetServer>) -> Handle<Image> {
    let n: i32 = rand::random_range(1..=36);
    let background_filename = format!("gfx/loadingscreens/load_{}.dds", n);
    asset_server.load(background_filename)
}
