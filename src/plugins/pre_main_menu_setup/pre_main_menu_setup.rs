use crate::{
    core::states::GameState,
    plugins::pre_main_menu_setup::systems::insert_cam_and_cube::insert_cam_and_cube,
};
use bevy::prelude::*;

pub struct PreMainMenuSetupPlugin;

impl Plugin for PreMainMenuSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PreMainMenuSetup), insert_cam_and_cube);
    }
}
