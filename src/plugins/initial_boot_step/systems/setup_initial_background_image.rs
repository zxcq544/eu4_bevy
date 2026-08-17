use crate::plugins::initial_boot_step::{
    components::initial_boot_entity::InitialBootEntity,
    resources::initial_booting_background_screen::InitialBootingBackgroundScreen,
};
use bevy::prelude::*;

pub fn setup_initial_background_image(
    mut commands: Commands,
    background_image_res: Res<InitialBootingBackgroundScreen>,
) {
    info!("Setting initial background image");
    commands.spawn((Camera2d::default(), InitialBootEntity));
    // TODO: check how to free resources and components and so on
    commands.spawn_scene(InitialBootEntity::as_scene(
        background_image_res.image.clone(),
    ));
}
