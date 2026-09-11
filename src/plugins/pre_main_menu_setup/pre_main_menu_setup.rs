use crate::{
    core::states::GameState,
    plugins::{
        main_menu::systems::{
            spawn_continue_block::spawn_continue_block,
            spawn_main_menu_scene::spawn_main_menu_scene,
        },
        options::systems::spawn_options_ui_main_entity::spawn_options_ui_main_entity,
        pre_main_menu_setup::systems::insert_cam_and_cube::insert_cam_and_cube,
    },
};
use bevy::prelude::*;

pub struct PreMainMenuSetupPlugin;

// TODO: refactor - move it to loading step so it doesn't hang main menu
impl Plugin for PreMainMenuSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::PreMainMenuSetup),
            (
                spawn_main_menu_scene,
                spawn_continue_block,
                spawn_options_ui_main_entity,
            ),
        );
        app.add_systems(OnEnter(GameState::PreMainMenuSetup), insert_cam_and_cube);
    }
}
