use bevy::prelude::*;

use crate::MainMenuBackground;

pub fn cleanup_initial_background(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuBackground>>,
) {
    for entity in query.iter() {
        // despawn_recursive deletes the parent and ALL nested children automatically
        commands.entity(entity).despawn();
        info!("background removed {:?}", entity);
    }
}
