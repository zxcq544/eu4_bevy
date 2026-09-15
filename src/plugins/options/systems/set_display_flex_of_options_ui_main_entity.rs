use crate::plugins::options::components::options_ui_main_entity::OptionsUiMainEntity;
use bevy::prelude::*;

pub fn set_display_flex_of_options_ui_main_entity(
    mut node_of_options_ui_main_entity_request: Query<&mut Node, With<OptionsUiMainEntity>>,
) {
    for mut node_of_options_ui_main_entity in &mut node_of_options_ui_main_entity_request {
        node_of_options_ui_main_entity.display = Display::Flex;
    }
}
