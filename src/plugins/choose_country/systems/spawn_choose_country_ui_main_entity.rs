use crate::{
    plugins::{
        choose_country::components::{
            bottom_middle_block_entity_for_choose_country::{
                CustomUiMaterial, CustomUiMaterialWithGlow,
            },
            choose_country_ui_main_entity::ChooseCountryUiMainEntity,
        },
        game_main::resources::game_ui_resources::GameUiResources,
    },
    shared::{
        country::{
            countries_resource::CountriesResource, country_data_from_json::CountriesDataFromJson,
        },
        fonts::fonts::FontHandles,
    },
};
use bevy::prelude::*;
use bevy_fluent::Localization;

pub fn spawn_choose_country_ui_main_entity(
    commands: Commands,
    localization_res: Res<Localization>,
    fonts: Res<FontHandles>,
    game_ui_resources: Res<GameUiResources>,
    ui_materials_res: ResMut<Assets<CustomUiMaterial>>,
    ui_materials_with_glow_res: ResMut<Assets<CustomUiMaterialWithGlow>>,
    countries_res: Res<CountriesResource>,
    countries_assets: Res<Assets<CountriesDataFromJson>>,
) {
    info!("Spawning choose country ui main entity");
    ChooseCountryUiMainEntity::spawn_using_commands(
        commands,
        &localization_res,
        &fonts,
        &game_ui_resources,
        ui_materials_res,
        ui_materials_with_glow_res,
        countries_res,
        countries_assets,
    );
}
