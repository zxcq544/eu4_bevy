use bevy::prelude::*;

use crate::setup_cursors;

pub struct InitialBootStepPlugin;
/// This plugin is responsible for setting up the initial boot step
/// This one sets default cursor and initial background and camera
impl Plugin for InitialBootStepPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cursors);
    }
}
