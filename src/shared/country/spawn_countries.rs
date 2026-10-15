use crate::shared::country::{
    countries_resource::CountriesResource, country::Country,
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
    let mut countries = Vec::with_capacity(countries_data.countries.len());
    for country_json_data in &countries_data.countries {
        countries.push(Country::from(country_json_data));
    }
    commands.spawn_batch(countries);
}
