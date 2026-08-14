use crate::{
    core::states::GameState,
    plugins::initial_boot_step::systems::{
        free_initial_background::cleanup_initial_background,
        load_background_image::load_background_image, load_cursors::load_cursors,
        start_loading_main_loading_step_background::start_loading_main_loading_step_background,
        whole_setup_step::whole_setup_step,
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
            (
                load_cursors,
                load_background_image,
                start_loading_main_loading_step_background,
            ),
        );
        app.add_systems(Update, whole_setup_step.run_if(in_state(GameState::Boot)));
        app.add_systems(OnExit(GameState::Boot), cleanup_initial_background);
    }
}
