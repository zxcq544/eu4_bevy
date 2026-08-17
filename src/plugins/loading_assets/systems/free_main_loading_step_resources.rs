use crate::plugins::loading_assets::{
    loading_assets::MainLoadingStepBackgroundImage,
    resources::loading_screen_tooltip_image::LoadingScreenTooltipImage,
};
use bevy::prelude::*;

pub fn free_main_loading_step_resources(
    mut commands: Commands,
    query: Query<Entity, With<MainLoadingStepBackgroundImage>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<LoadingScreenTooltipImage>();
    commands.remove_resource::<MainLoadingStepBackgroundImage>();
}
