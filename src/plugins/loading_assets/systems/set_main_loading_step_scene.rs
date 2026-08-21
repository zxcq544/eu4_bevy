use crate::{
    core::states::GameState,
    plugins::loading_assets::{
        components::main_loading_step_main_entity::MainLoadingStepMainEntity,
        resources::{
            loading_screen_tooltip_image::LoadingScreenTooltipImage,
            main_loading_step_background_image::MainLoadingStepBackgroundImage,
        },
    },
};
use bevy::prelude::*;
use bevy_fluent::Localization;
use fonts::FontHandles;

pub fn set_main_loading_step_scene(
    mut commands: Commands,
    main_image_res: Res<MainLoadingStepBackgroundImage>,
    loading_screen_tooltip_image_res: Res<LoadingScreenTooltipImage>,
    fonts_res: Res<FontHandles>,
    current_state: Res<State<GameState>>,
    localization_res: Res<Localization>,
) {
    info!("current state is {:?}", current_state.get());
    info!("Setting main loading step background image");
    // info!("Main loading step background image is loaded");
    commands.spawn((Camera2d::default(), MainLoadingStepMainEntity));
    // TODO: try to convert to bsn! and check how to free resources and components and so on
    commands.spawn_scene_list(MainLoadingStepMainEntity::as_scene_list(
        main_image_res,
        loading_screen_tooltip_image_res,
        fonts_res,
        localization_res,
    ));
}
