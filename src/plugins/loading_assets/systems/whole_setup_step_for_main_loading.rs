use crate::{
    core::states::GameState,
    plugins::{
        loading_assets::resources::timer_for_main_loading_step::TimerForMainLoadingStep,
        main_menu::resources::main_menu_all_images::MainMenuAllImages,
        sound_effects::resources::button_click_sound_effects::ButtonClickSoundEffects,
    },
};
use bevy::prelude::*;

pub fn whole_setup_step_for_main_loading(
    asset_server: Res<AssetServer>,
    mut timer: ResMut<TimerForMainLoadingStep>,
    time: Res<Time>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    main_menu_all_images: Res<MainMenuAllImages>,
    sound_effects: Res<ButtonClickSoundEffects>,
) {
    timer.timer.tick(time.delta());

    if asset_server.is_loaded_with_dependencies(&main_menu_all_images.main_menu_background_image)
        && asset_server.is_loaded_with_dependencies(&sound_effects.button_click_general)
        && timer.timer.just_finished()
    {
        info!("current state is {:?}", current_state.get());
        info!("moving to main menu state");
        next_state.set(GameState::MainMenu);
    }
}
