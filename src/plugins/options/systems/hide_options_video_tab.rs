use crate::plugins::options::components::options_middle_block::video_tab::OptionsUiVideoTab;
use bevy::prelude::*;

pub fn hide_options_video_tab(
    mut visibility_of_options_video_tab_query: Query<&mut Visibility, With<OptionsUiVideoTab>>,
) {
    for mut visibility_of_options_video_tab in &mut visibility_of_options_video_tab_query {
        *visibility_of_options_video_tab = Visibility::Hidden;
    }
}
