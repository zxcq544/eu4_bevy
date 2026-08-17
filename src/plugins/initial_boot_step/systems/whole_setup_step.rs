use bevy::prelude::*;

use crate::{
    core::states::GameState,
    plugins::{
        initial_boot_step::resources::{
            cursor_handles::CursorHandles, initial_boot_step_timer::InitialBootStepTimer,
            initial_booting_background_screen::InitialBootingBackgroundScreen,
        },
        loading_assets::{
            loading_assets::MainLoadingStepBackgroundImage,
            resources::loading_screen_tooltip_image::LoadingScreenTooltipImage,
        },
    },
};

pub fn whole_setup_step(
    asset_server: Res<AssetServer>,
    cursors: Res<CursorHandles>,
    background_image: Res<InitialBootingBackgroundScreen>,
    loading_screen_tooltip_image: Res<LoadingScreenTooltipImage>,
    main_background: Res<MainLoadingStepBackgroundImage>,
    mut next_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<InitialBootStepTimer>,
    time: Res<Time>,
) {
    // info!("Whole setup step");
    timer.timer.tick(time.delta());
    if asset_server.is_loaded_with_dependencies(&cursors.normal)
        && asset_server.is_loaded_with_dependencies(&background_image.image)
        && asset_server.is_loaded_with_dependencies(&loading_screen_tooltip_image.image)
        && asset_server.is_loaded_with_dependencies(&main_background.image)
        && timer.timer.just_finished()
    {
        next_state.set(GameState::LoadingAssets);
    }
}
