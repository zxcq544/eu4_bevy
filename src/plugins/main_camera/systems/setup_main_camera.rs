use crate::plugins::main_camera::{
    components::main_camera_entity::MainCameraEntity, resources::camera_config::CameraConfig,
};
use bevy::prelude::*;

pub fn setup_main_camera(mut commands: Commands, config: Res<CameraConfig>) {
    let camera_x = 0.0;
    let camera_z = 0.0;
    let camera_y = config.max_height; // e.g., 1024.0

    // Calculate the exact starting pitch based on initial height
    let zoom_t = (config.max_height - camera_y) / (config.max_height - config.min_height);
    let initial_pitch = config.min_pitch + zoom_t * (config.max_pitch - config.min_pitch);

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(camera_x, camera_y, camera_z)
            .with_rotation(Quat::from_rotation_x(initial_pitch)),
        MainCameraEntity,
    ));
}
