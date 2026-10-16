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
        country::country::{CountryIndexInMenu, CountryInfo, CountryIsRecommendedInMenu},
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
    countries_recommended_for_main_menu_query: Query<
        &CountryInfo,
        With<CountryIsRecommendedInMenu>,
    >,
    countries_for_main_menu_but_not_recommended_query: Query<
        &CountryInfo,
        (
            With<CountryIndexInMenu>,
            Without<CountryIsRecommendedInMenu>,
        ),
    >,
) {
    info!("Spawning choose country ui main entity");
    ChooseCountryUiMainEntity::spawn_using_commands(
        commands,
        &localization_res,
        &fonts,
        &game_ui_resources,
        ui_materials_res,
        ui_materials_with_glow_res,
        countries_recommended_for_main_menu_query,
        countries_for_main_menu_but_not_recommended_query,
    );
}
