use bevy::prelude::*;
use crate::{
    core::states::GameState,
    plugins::main_camera::{
        resources::camera_config::CameraConfig,
        systems::camera_control_system::camera_control_system,
    },
};

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraConfig>();
        app.add_systems(
            Update,
            camera_control_system.run_if(in_state(GameState::MainMenu)),
        );
    }
}
