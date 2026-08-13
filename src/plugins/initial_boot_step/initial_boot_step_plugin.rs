use crate::{
    core::states::GameState,
    plugins::initial_boot_step::systems::{
        check_if_time_elapsed::check_if_time_elapsed,
        free_initial_background::cleanup_initial_background, setup_cursors::setup_cursors,
        setup_initial_background_image::setup_initial_background_image,
        setup_timeout::setup_timeout,
        start_loading_main_loading_step_background::start_loading_main_loading_step_background,
    },
};
use bevy::prelude::*;

pub struct InitialBootStepPlugin;
/// This plugin is responsible for setting up the initial boot step
/// This one sets default cursor and initial background and camera
impl Plugin for InitialBootStepPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Boot),
            (setup_initial_background_image, setup_cursors, setup_timeout),
        );
        app.add_systems(
            OnEnter(GameState::Boot),
            start_loading_main_loading_step_background,
        );
        app.add_systems(Update, check_if_time_elapsed);
        app.add_systems(OnExit(GameState::Boot), cleanup_initial_background);
    }
}
