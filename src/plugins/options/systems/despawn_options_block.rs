use crate::plugins::options::components::options_background_entity::OptionsBackgroundEntity;
use bevy::prelude::*;

pub fn despawn_options_block(
    mut commands: Commands,
    query: Query<Entity, With<OptionsBackgroundEntity>>,
) {
    info!("Despawning options block");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    // DO not remove resouces for options here because they will be used in main game options screen
    // At least for now
}
