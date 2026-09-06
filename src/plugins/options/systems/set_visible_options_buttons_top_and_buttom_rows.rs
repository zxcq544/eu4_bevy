use crate::plugins::options::components::options_top_tabs_row_entity::OptionsButtonsTopAndBottomRows;
use bevy::prelude::*;

pub fn set_visible_options_block_top_and_bottom_buttons(
    mut visibility_of_options_block_top_and_bottom_buttons_query: Query<
        &mut Visibility,
        With<OptionsButtonsTopAndBottomRows>,
    >,
) {
    for mut visibility_of_options_block_top_and_bottom_buttons in
        &mut visibility_of_options_block_top_and_bottom_buttons_query
    {
        *visibility_of_options_block_top_and_bottom_buttons = Visibility::Visible;
    }
}
