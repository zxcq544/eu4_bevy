use crate::shared::country::{
    countries_resource::CountriesResource,
    country::{
        Country, CountryBundle, CountryInMainMenuButNotRecommendedBundle, CountryIndexInMenu,
        CountryInfo, CountryIsRecommendedInMenu, CountryRecommendedForMainMenuBundle,
    },
    country_data_from_json::CountriesDataFromJson,
};
use bevy::{
    asset::Assets,
    ecs::system::{Commands, Res},
    log::warn,
};

pub fn spawn_countries(
    mut commands: Commands,
    countries_res: Res<CountriesResource>,
    countries_assets: Res<Assets<CountriesDataFromJson>>,
) {
    let Some(countries_data) = countries_assets.get(&countries_res.countries) else {
        warn!("Countries data asset not found yet.");
        return;
    };
    let mut countries_not_in_main_menu: Vec<CountryBundle> = Vec::new();
    let mut not_recommended_countries_in_main_menu: Vec<CountryInMainMenuButNotRecommendedBundle> =
        Vec::new();
    let mut recommended_countries_in_main_menu: Vec<CountryRecommendedForMainMenuBundle> =
        Vec::new();
    for country_data in &countries_data.countries {
        let country_info = CountryInfo {
            name: country_data.name.clone(),
            short_name_tag_3_letters: country_data.short_name_tag_3_letters.clone(),
            flag_index: country_data.flag_index,
        };
        match (country_data.index_in_menu, country_data.recommended_in_menu) {
            (Some(index), Some(true)) => {
                recommended_countries_in_main_menu.push(CountryRecommendedForMainMenuBundle {
                    marker: Country,
                    country_info,
                    country_is_recommended_in_menu: CountryIsRecommendedInMenu,
                    country_index_in_menu: CountryIndexInMenu(index),
                });
            }
            (Some(index), _) => {
                not_recommended_countries_in_main_menu.push(
                    CountryInMainMenuButNotRecommendedBundle {
                        marker: Country,
                        country_info,
                        country_index_in_menu: CountryIndexInMenu(index),
                    },
                );
            }
            _ => {
                countries_not_in_main_menu.push(CountryBundle {
                    marker: Country,
                    country_info,
                });
            }
        }
    }
    not_recommended_countries_in_main_menu.sort_unstable_by_key(|c| c.country_index_in_menu.0);
    recommended_countries_in_main_menu.sort_unstable_by_key(|c| c.country_index_in_menu.0);

    commands.spawn_batch(countries_not_in_main_menu);
    commands.spawn_batch(not_recommended_countries_in_main_menu);
    commands.spawn_batch(recommended_countries_in_main_menu);
}
