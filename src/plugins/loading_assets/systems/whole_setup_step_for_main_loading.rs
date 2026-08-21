use crate::{
    core::states::GameState,
    plugins::{
        loading_assets::resources::timer_for_main_loading_step::TimerForMainLoadingStep,
        main_menu::resources::background_image_of_main_menu::BackgroundImageOfMainMenu,
    },
};
use bevy::prelude::*;

pub fn whole_setup_step_for_main_loading(
    asset_server: Res<AssetServer>,
    mut timer: ResMut<TimerForMainLoadingStep>,
    time: Res<Time>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    background_image: Res<BackgroundImageOfMainMenu>,
) {
    timer.timer.tick(time.delta());

    if asset_server.is_loaded_with_dependencies(&background_image.image)
        && timer.timer.just_finished()
    {
        info!("current state is {:?}", current_state.get());
        info!("moving to main menu state");
        next_state.set(GameState::MainMenu);
    }
}
