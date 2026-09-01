use crate::plugins::main_menu::components::options_entity::OptionsEntity;
use bevy::prelude::*;
use bevy_fluent::Localization;
use fonts::FontHandles;

pub fn spawn_options_block(
    mut commands: Commands,
    fonts: Res<FontHandles>,
    localization_res: Res<Localization>,
) {
    commands.spawn_scene_list(OptionsEntity::as_scene_list(&localization_res, &fonts));
}
