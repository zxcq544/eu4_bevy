use crate::plugins::{
    game_main::resources::game_ui_resources::GameUiResources,
    options::{
        components::options_ui_main_entity::OptionsUiMainEntity,
        resources::options_images::OptionsImages,
    },
};
use bevy::prelude::*;
use bevy_fluent::Localization;
use fonts::FontHandles;
use settings::Settings;

pub fn spawn_options_ui_main_entity(
    settings: ResMut<Settings>,
    commands: Commands,
    fonts: Res<FontHandles>,
    localization_res: Res<Localization>,
    options_images: Res<OptionsImages>,
    ui_resources: Res<GameUiResources>,
) {
    info!("Spawning options ui main entity");
    OptionsUiMainEntity::spawn_using_commands(
        settings,
        commands,
        &localization_res,
        &fonts,
        &options_images,
        &ui_resources,
    );
}
