use crate::plugins::loading_assets::{
    components::main_loading_step_main_entity::MainLoadingStepMainEntity,
    loading_assets::MainLoadingStepBackgroundImage,
    resources::loading_screen_tooltip_image::LoadingScreenTooltipImage,
};
use bevy::prelude::*;

pub fn free_main_loading_step_resources(
    mut commands: Commands,
    query: Query<Entity, With<MainLoadingStepMainEntity>>,
) {
    info!("Freeing main loading step resources");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<LoadingScreenTooltipImage>();
    commands.remove_resource::<MainLoadingStepBackgroundImage>();
}
