use crate::plugins::initial_boot_step::components::main_menu_background::MainMenuBackground;
use bevy::prelude::*;

pub fn cleanup_initial_background(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuBackground>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
        info!("background removed {:?}", entity);
    }
}
