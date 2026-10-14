use crate::shared::country::country_data_from_json::CountryDataFromJson;

pub struct Country {
    pub name: String,
    pub short_name_tag_3_letters: String,
    pub flag_index: u32,
    pub recommended_in_menu: Option<bool>,
    pub index_in_menu: Option<u32>,
}

impl From<&CountryDataFromJson> for Country {
    fn from(data: &CountryDataFromJson) -> Self {
        Self {
            name: data.name.clone(),
            flag_index: data.flag_index,
            short_name_tag_3_letters: data.short_name_tag_3_letters.clone(),
            recommended_in_menu: data.recommended_in_menu,
            index_in_menu: data.index_in_menu,
        }
    }
}
