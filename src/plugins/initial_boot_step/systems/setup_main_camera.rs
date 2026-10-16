use crate::plugins::pre_main_menu_setup::systems::insert_cam_and_cube::MainCameraEntity;
use bevy::prelude::*;

pub fn setup_main_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(), MainCameraEntity));
}
