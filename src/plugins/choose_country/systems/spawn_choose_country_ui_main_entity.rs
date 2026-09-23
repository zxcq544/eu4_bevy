use crate::{
    plugins::{
        choose_country::components::{
            bottom_middle_block_entity_for_choose_country::CustomUiMaterial,
            choose_country_ui_main_entity::ChooseCountryUiMainEntity,
        },
        game_main::resources::game_ui_resources::GameUiResources,
    },
    shared::fonts::fonts::FontHandles,
};
use bevy::prelude::*;
use bevy_fluent::Localization;

pub fn spawn_choose_country_ui_main_entity(
    commands: Commands,
    localization_res: Res<Localization>,
    fonts: Res<FontHandles>,
    game_ui_resources: Res<GameUiResources>,
    ui_materials_res: ResMut<Assets<CustomUiMaterial>>,
) {
    info!("Spawning choose country ui main entity");
    ChooseCountryUiMainEntity::spawn_using_commands(
        commands,
        &localization_res,
        &fonts,
        &game_ui_resources,
        ui_materials_res,
    );
}
