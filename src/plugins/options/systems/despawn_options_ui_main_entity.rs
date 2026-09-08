use crate::plugins::options::components::options_ui_main_entity::OptionsUiMainEntity;
use bevy::prelude::*;

pub fn despawn_options_top_and_bottom_buttons(
    mut commands: Commands,
    query: Query<Entity, With<OptionsUiMainEntity>>,
) {
    info!("Despawning options ui main entity");
    // DO not remove resouces for options buttons here because they will be used in main game options screen
    // At least for now
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
