use crate::{
    core::states::GameState,
    plugins::{
        choose_country::systems::spawn_choose_country_ui_main_entity::spawn_choose_country_ui_main_entity,
        loading_assets::systems::free_main_loading_step_resources::free_main_loading_step_resources,
        main_menu::{
            components::main_menu_entity::MainMenuEntity,
            systems::{
                spawn_continue_block::spawn_continue_block,
                spawn_main_menu_scene::spawn_main_menu_scene,
            },
        },
        options::systems::spawn_options_ui_main_entity::spawn_options_ui_main_entity,
        pre_main_menu_setup::systems::insert_cam_and_cube::insert_cam_and_cube,
    },
    shared::country::spawn_countries::spawn_countries,
};
use bevy::{ecs::query::SpawnDetails, prelude::*};

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
            ),
        );
        app.add_systems(
            OnEnter(GameState::PreMainMenuSetup),
            (spawn_continue_block, spawn_choose_country_ui_main_entity).after(spawn_countries),
        );
        app.add_systems(OnEnter(GameState::PreMainMenuSetup), insert_cam_and_cube);
        app.add_systems(
            Update,
            check_if_menu_is_ready.run_if(in_state(GameState::PreMainMenuSetup)),
        );
        // Main loading step image stays active until we spawn everything that we loaded at that step
        app.add_systems(
            OnExit(GameState::PreMainMenuSetup),
            free_main_loading_step_resources,
        );
    }
}

fn check_if_menu_is_ready(
    query: Query<(&MainMenuEntity, SpawnDetails)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (_entity, spawn_details) in query.iter() {
        if spawn_details.is_spawned() {
            info!("All entities spawned successfully! Moving to MainMenu state.");
            next_state.set(GameState::MainMenu);
        }
    }
}
