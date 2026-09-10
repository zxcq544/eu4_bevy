use crate::plugins::options::components::options_middle_block::video_tab::OptionsUiVideoTab;
use bevy::prelude::*;

pub fn hide_options_video_tab(
    mut visibility_of_options_video_tab_query: Query<&mut Node, With<OptionsUiVideoTab>>,
) {
    for mut node in &mut visibility_of_options_video_tab_query {
        node.display = Display::None;
    }
}
