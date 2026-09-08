use crate::plugins::options::components::options_ui_main_entity::OptionsUiMainEntity;
use bevy::prelude::*;

pub fn set_visible_options_ui_main_entity(
    mut visibility_of_options_ui_main_entity: Query<
        &mut Visibility,
        With<OptionsUiMainEntity>,
    >,
) {
    for mut visibility_of_options_ui_main_entity in
        &mut visibility_of_options_ui_main_entity
    {
        *visibility_of_options_ui_main_entity = Visibility::Visible;
    }
}
