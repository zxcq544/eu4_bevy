use crate::plugins::main_menu::{
    components::main_menu_entity::MainMenuEntity,
    resources::{exit_delay_timer::ExitDelayTimer, main_menu_all_images::MainMenuAllImages},
};
use bevy::prelude::*;

pub fn free_main_menu_entity_and_resources(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuEntity>>,
) {
    info!("Freeing main menu entity and resources");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<MainMenuAllImages>();
    commands.remove_resource::<ExitDelayTimer>();
}
