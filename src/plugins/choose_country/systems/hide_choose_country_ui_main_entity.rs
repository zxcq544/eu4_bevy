use crate::plugins::choose_country::components::choose_country_ui_main_entity::ChooseCountryUiMainEntity;
use bevy::prelude::*;

pub fn hide_choose_country_ui_main_entity(
    mut node_of_choose_country_ui_main_entity_request: Query<
        &mut Node,
        With<ChooseCountryUiMainEntity>,
    >,
) {
    for mut node_of_choose_country_ui_main_entity in
        &mut node_of_choose_country_ui_main_entity_request
    {
        node_of_choose_country_ui_main_entity.display = Display::None;
    }
}
