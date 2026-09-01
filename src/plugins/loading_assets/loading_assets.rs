use crate::{
    core::states::GameState,
    plugins::loading_assets::systems::{
        free_main_loading_step_resources::free_main_loading_step_resources,
        load_all_game_sound_effects::load_all_game_sound_effects,
        load_resources_for_main_menu::{
            load_images_for_main_menu::load_images_for_main_menu,
            load_images_for_options::load_images_for_options,
        },
        set_main_loading_step_scene::set_main_loading_step_scene,
        start_timer_for_main_loading_step::start_timer_for_main_loading_step,
        whole_setup_step_for_main_loading::whole_setup_step_for_main_loading,
    },
};
use bevy::prelude::*;

pub struct LoadingAssetsPlugin;

impl Plugin for LoadingAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::LoadingAssets),
            (
                start_timer_for_main_loading_step,
                set_main_loading_step_scene,
            )
                .chain(),
        );
        app.add_systems(
            OnEnter(GameState::LoadingAssets),
            (
                load_images_for_main_menu,
                load_all_game_sound_effects,
                load_images_for_options,
            ),
        );
        app.add_systems(
            Update,
            whole_setup_step_for_main_loading.run_if(in_state(GameState::LoadingAssets)),
        );
        app.add_systems(
            OnExit(GameState::LoadingAssets),
            free_main_loading_step_resources,
        );
    }
}
