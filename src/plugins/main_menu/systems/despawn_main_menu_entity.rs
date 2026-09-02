use crate::plugins::main_menu::components::main_menu_entity::MainMenuEntity;
use bevy::prelude::*;

pub fn despawn_main_menu_entity(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuEntity>>,
) {
    info!("Freeing main menu entity and timer");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    // Do not unload images here because when moving from options to main menu we will need them again
    // commands.remove_resource::<MainMenuAllImages>();
    // commands.remove_resource::<ExitDelayTimer>();
}
