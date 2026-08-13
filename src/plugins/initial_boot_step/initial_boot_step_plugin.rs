use bevy::prelude::*;

use crate::{
    check_if_time_elapsed, cleanup_initial_background, setup_cursors, setup_initial_background_image, setup_timeout, states::GameState,
};

pub struct InitialBootStepPlugin;
/// This plugin is responsible for setting up the initial boot step
/// This one sets default cursor and initial background and camera
impl Plugin for InitialBootStepPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Boot), setup_cursors)
            .add_systems(OnEnter(GameState::Boot), setup_initial_background_image);
        app.add_systems(OnEnter(GameState::Boot), setup_timeout);
        app.add_systems(Update, check_if_time_elapsed);
        app.add_systems(OnExit(GameState::Boot), cleanup_initial_background);
    }
}
