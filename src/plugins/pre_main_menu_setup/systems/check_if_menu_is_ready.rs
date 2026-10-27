use crate::plugins::main_menu::components::main_menu_entity::MainMenuEntity;
use bevy::{ecs::query::SpawnDetails, prelude::*};

pub fn check_if_menu_is_ready(query: Query<(&MainMenuEntity, SpawnDetails)>) -> bool {
    for (_entity, spawn_details) in query.iter() {
        if spawn_details.is_spawned() {
            info!("Main menu spawned successfully! Moving to MainMenu state.");
            return true;
        }
    }
    return false;
}
