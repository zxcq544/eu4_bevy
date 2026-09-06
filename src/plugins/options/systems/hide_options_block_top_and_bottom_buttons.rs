use crate::plugins::options::components::options_top_tabs_row_entity::OptionsButtonsTopAndBottomRows;
use bevy::prelude::*;

pub fn hide_options_block_top_and_bottom_buttons(
    mut visibility_of_options_blocks_query: Query<
        &mut Visibility,
        With<OptionsButtonsTopAndBottomRows>,
    >,
) {
    for mut visibility_of_options_block in &mut visibility_of_options_blocks_query {
        *visibility_of_options_block = Visibility::Hidden;
    }
}
