#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Clone)]
pub struct CountryDataFromJson {
    pub name: String,
    pub short_name_tag_3_letters: String,
    pub flag_index: u32,
    pub recommended_in_menu: Option<bool>,
    pub index_in_menu: Option<u32>,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Clone)]
pub struct CountriesDataFromJson {
    pub countries: Vec<CountryDataFromJson>,
}
