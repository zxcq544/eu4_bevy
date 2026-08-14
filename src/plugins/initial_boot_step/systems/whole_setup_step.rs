use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    core::states::GameState,
    plugins::initial_boot_step::{
        resources::{
            cursor_handles::CursorHandles,
            initial_booting_background_screen::InitialBootingBackgroundScreen,
        },
        systems::{
            setup_cursors::setup_cursors,
            setup_initial_background_image::setup_initial_background_image,
        },
    },
};

pub fn whole_setup_step(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<Entity, With<PrimaryWindow>>,
    cursors: Res<CursorHandles>,
    background_image: Res<InitialBootingBackgroundScreen>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("Whole setup step");
    if asset_server.is_loaded_with_dependencies(&cursors.normal)
        && asset_server.is_loaded_with_dependencies(&background_image.image)
    {
        setup_cursors(&mut commands, window, asset_server, cursors);
        setup_initial_background_image(&mut commands, background_image);
        next_state.set(GameState::LoadingAssets);
    }
}
