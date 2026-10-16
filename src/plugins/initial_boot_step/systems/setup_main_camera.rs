use crate::shared::main_camera::main_camera::MainCameraEntity;
use bevy::prelude::*;

pub fn setup_main_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(), MainCameraEntity));
}
