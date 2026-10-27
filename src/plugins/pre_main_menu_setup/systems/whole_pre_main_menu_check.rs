use crate::{
    core::states::GameState,
    plugins::{
        main_menu::components::main_menu_entity::MainMenuEntity,
        map::systems::generate_map_mesh::MapRootEntity,
        pre_main_menu_setup::systems::{
            check_if_map_mesh_is_ready::check_if_map_mesh_is_ready,
            check_if_menu_is_ready::check_if_menu_is_ready,
        },
    },
};
use bevy::{
    ecs::{
        query::SpawnDetails,
        system::{Query, ResMut},
    },
    log::info,
    state::state::NextState,
};

pub fn whole_pre_main_menu_check(
    map_mesh_query: Query<(&MapRootEntity, SpawnDetails)>,
    menu_query: Query<(&MainMenuEntity, SpawnDetails)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if check_if_map_mesh_is_ready(map_mesh_query) && check_if_menu_is_ready(menu_query) {
        info!("All entities spawned successfully! Moving to MainMenu state.");
        next_state.set(GameState::MainMenu);
    }
}
