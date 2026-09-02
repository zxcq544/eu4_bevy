use crate::plugins::main_menu::{
    components::options_entity::OptionsEntity, resources::options_images::OptionsImages,
};
use bevy::prelude::*;
use bevy_fluent::Localization;
use fonts::FontHandles;

pub fn spawn_options_block(
    mut commands: Commands,
    fonts: Res<FontHandles>,
    localization_res: Res<Localization>,
    options_images: Res<OptionsImages>,
) {
    info!("Spawning options block");
    commands.spawn_scene_list(OptionsEntity::as_scene_list(
        &options_images,
        &localization_res,
        &fonts,
    ));
}
