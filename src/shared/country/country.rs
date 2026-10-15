use bevy::ecs::{bundle::Bundle, component::Component};

#[derive(Component, Debug, Clone)]
pub struct Country;

#[derive(Component, Debug, Clone)]
pub struct CountryInfo {
    pub name: String,
    pub short_name_tag_3_letters: String,
    pub flag_index: u32,
}

#[derive(Component, Debug)]
pub struct CountryIsRecommendedInMenu;

#[derive(Component, Debug)]
pub struct CountryIndexInMenu(pub u32);

#[derive(Bundle, Debug)]
pub struct CountryBundle {
    pub marker: Country,
    pub country_info: CountryInfo,
}

#[derive(Bundle, Debug)]
pub struct CountryRecommendedForMainMenuBundle {
    pub marker: Country,
    pub country_info: CountryInfo,
    pub country_is_recommended_in_menu: CountryIsRecommendedInMenu,
    pub country_index_in_menu: CountryIndexInMenu,
}

#[derive(Bundle, Debug)]
pub struct CountryInMainMenuButNotRecommendedBundle {
    pub marker: Country,
    pub country_info: CountryInfo,
    pub country_index_in_menu: CountryIndexInMenu,
}

// impl From<&CountryDataFromJson> for Country {
//     fn from(data: &CountryDataFromJson) -> Self {
//         Self {
//             name: data.name.clone(),
//             flag_index: data.flag_index,
//             short_name_tag_3_letters: data.short_name_tag_3_letters.clone(),
//             recommended_in_menu: data.recommended_in_menu,
//             index_in_menu: data.index_in_menu,
//         }
//     }
// }
