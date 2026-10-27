use crate::plugins::map::systems::generate_map_mesh::MapRootEntity;
use bevy::{ecs::query::SpawnDetails, prelude::*};

pub fn check_if_map_mesh_is_ready(query: Query<(&MapRootEntity, SpawnDetails)>) -> bool {
    for (_entity, spawn_details) in query.iter() {
        if spawn_details.is_spawned() {
            info!("Map mesh spawned successfully! Moving to MainMenu state.");
            return true;
        }
    }
    return false;
}
