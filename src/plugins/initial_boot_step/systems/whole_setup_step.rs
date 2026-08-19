use crate::{
    core::states::GameState,
    plugins::{
        initial_boot_step::{
            resources::{
                cursor_handles::CursorHandles, initial_boot_step_timer::InitialBootStepTimer,
                initial_booting_background_screen::InitialBootingBackgroundScreen,
                locale_folder::LocaleFolder,
            },
            systems::setup_localisation::setup_localisation,
        },
        loading_assets::{
            loading_assets::MainLoadingStepBackgroundImage,
            resources::loading_screen_tooltip_image::LoadingScreenTooltipImage,
        },
    },
};
use bevy::prelude::*;
use bevy_fluent::LocalizationBuilder;
use fonts::FontHandles;

pub fn whole_setup_step(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cursors: Res<CursorHandles>,
    fonts: Res<FontHandles>,
    background_image: Res<InitialBootingBackgroundScreen>,
    loading_screen_tooltip_image: Res<LoadingScreenTooltipImage>,
    main_background: Res<MainLoadingStepBackgroundImage>,
    locale_folder_res: Res<LocaleFolder>,
    localization_builder: LocalizationBuilder,
    mut next_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<InitialBootStepTimer>,
    time: Res<Time>,
) {
    // info!("Whole setup step");
    timer.timer.tick(time.delta());
    if asset_server.is_loaded_with_dependencies(&cursors.normal)
        && asset_server.is_loaded_with_dependencies(&fonts.loading_screen_tooltip_font)
        && asset_server.is_loaded_with_dependencies(&fonts.loading_screen_loading_text_font)
        && asset_server.is_loaded_with_dependencies(&background_image.image)
        && asset_server.is_loaded_with_dependencies(&loading_screen_tooltip_image.image)
        && asset_server.is_loaded_with_dependencies(&main_background.image)
        && asset_server.is_loaded_with_dependencies(&locale_folder_res.folder)
        && timer.timer.just_finished()
    {
        setup_localisation(
            &mut commands,
            localization_builder,
            locale_folder_res,
            asset_server,
        );
        next_state.set(GameState::LoadingAssets);
    }
}
