use crate::{
    core::states::GameState,
    plugins::{
        choose_country::systems::spawn_choose_country_ui_main_entity::spawn_choose_country_ui_main_entity,
        loading_assets::systems::free_main_loading_step_resources::free_main_loading_step_resources,
        main_menu::systems::spawn_main_menu_scene::spawn_main_menu_scene,
        map::systems::generate_map_mesh::setup_terrain_system,
        options::systems::spawn_options_ui_main_entity::spawn_options_ui_main_entity,
        pre_main_menu_setup::systems::{
            insert_cube_and_sun::insert_cube_and_sun,
            whole_pre_main_menu_check::whole_pre_main_menu_check,
        },
    },
    shared::country::spawn_countries::spawn_countries,
};
use bevy::prelude::*;

pub struct PreMainMenuSetupPlugin;

// This step is used to spawn everything.
// While we spawn everything loading step image must stay on
// so we free resources of main loading state when we exit this state(it's just a few bg images)
impl Plugin for PreMainMenuSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::PreMainMenuSetup),
            (
                spawn_countries,
                spawn_main_menu_scene,
                spawn_options_ui_main_entity,
                setup_terrain_system,
            ),
        );
        app.add_systems(
            OnEnter(GameState::PreMainMenuSetup),
            (spawn_choose_country_ui_main_entity).after(spawn_countries),
        );
        app.add_systems(OnEnter(GameState::PreMainMenuSetup), insert_cube_and_sun);
        app.add_systems(
            Update,
            whole_pre_main_menu_check.run_if(in_state(GameState::PreMainMenuSetup)),
        );
        // Main loading step image stays active until we spawn everything that we loaded at that step
        app.add_systems(
            OnExit(GameState::PreMainMenuSetup),
            free_main_loading_step_resources,
        );
    }
}
