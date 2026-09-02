use crate::plugins::main_menu::components::continue_game_entity::ContinueGameEntity;
use bevy::prelude::*;

pub fn despawn_continue_block(
    mut commands: Commands,
    query: Query<Entity, With<ContinueGameEntity>>,
) {
    info!("Despawning continue block");
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
