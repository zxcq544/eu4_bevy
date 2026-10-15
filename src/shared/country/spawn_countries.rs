use crate::shared::country::{
    countries_resource::CountriesResource,
    country::{
        Country, CountryBundle, CountryIndexInMenu, CountryInfo, CountryIsRecommendedInMenu,
    },
    country_data_from_json::CountriesDataFromJson,
};
use bevy::{
    asset::Assets,
    ecs::system::{Commands, Res},
};

pub fn spawn_countries(
    mut commands: Commands,
    countries_res: Res<CountriesResource>,
    countries_assets: Res<Assets<CountriesDataFromJson>>,
) {
    let Some(countries_data) = countries_assets.get(&countries_res.countries) else {
        panic!("Countries data asset not found");
    };
    let mut countries_not_in_main_menu: Vec<CountryBundle> = Vec::new();
    let mut not_recommended_countries_in_main_menu: Vec<(CountryBundle, u32)> = Vec::new();
    let mut recommended_countries_in_main_menu: Vec<(CountryBundle, u32)> = Vec::new();
    for country_data in &countries_data.countries {
        let country_bundle = CountryBundle {
            marker: Country,
            country_info: CountryInfo {
                name: country_data.name.clone(),
                short_name_tag_3_letters: country_data.short_name_tag_3_letters.clone(),
                flag_index: country_data.flag_index,
            },
        };
        if country_data.index_in_menu.is_some() && country_data.recommended_in_menu.is_none() {
            not_recommended_countries_in_main_menu
                .push((country_bundle, country_data.index_in_menu.unwrap()));
        } else if country_data.recommended_in_menu == Some(true)
            && country_data.index_in_menu.is_some()
        {
            recommended_countries_in_main_menu
                .push((country_bundle, country_data.index_in_menu.unwrap()));
        } else {
            countries_not_in_main_menu.push(country_bundle);
        }
    }
    commands.spawn_batch(countries_not_in_main_menu);
    not_recommended_countries_in_main_menu.sort_by_key(|(_, index_in_menu)| *index_in_menu);
    for (not_recommended_country_in_main_menu, index_in_menu) in
        not_recommended_countries_in_main_menu
    {
        commands.spawn((
            not_recommended_country_in_main_menu,
            CountryIndexInMenu(index_in_menu),
        ));
    }
    recommended_countries_in_main_menu.sort_by_key(|(_, index_in_menu)| *index_in_menu);
    for (recommended_country_in_main_menu, index_in_menu) in recommended_countries_in_main_menu {
        commands.spawn((
            recommended_country_in_main_menu,
            CountryIndexInMenu(index_in_menu),
            CountryIsRecommendedInMenu,
        ));
    }
}
