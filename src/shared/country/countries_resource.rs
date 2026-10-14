use crate::shared::country::country_data_from_json::CountriesDataFromJson;
use bevy::{asset::Handle, ecs::resource::Resource};

#[derive(Resource)]
pub struct CountriesResource {
    pub countries: Handle<CountriesDataFromJson>,
}
