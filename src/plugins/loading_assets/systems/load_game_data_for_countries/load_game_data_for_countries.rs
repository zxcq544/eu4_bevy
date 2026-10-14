use crate::shared::country::countries_resource::CountriesResource;
use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

pub fn load_game_data_for_countries(mut commands: Commands, asset_server: Res<AssetServer>) {
    let countries = asset_server.load("data/result_countries.json");
    commands.insert_resource(CountriesResource {
        countries: countries,
    });
}
