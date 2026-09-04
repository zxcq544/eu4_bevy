use crate::plugins::options::{
    components::options_top_tabs_row_entity::OptionsTopTabsRowEntity,
    resources::options_images::OptionsImages,
};
use bevy::prelude::*;
use bevy_fluent::Localization;
use fonts::FontHandles;

pub fn spawn_options_top_and_bottom_buttons(
    mut commands: Commands,
    fonts: Res<FontHandles>,
    localization_res: Res<Localization>,
    options_images: Res<OptionsImages>,
) {
    info!("Spawning options top and bottom buttons");
    commands.spawn_scene_list(OptionsTopTabsRowEntity::as_scene_list(
        &localization_res,
        &fonts,
        &options_images,
    ));
}
