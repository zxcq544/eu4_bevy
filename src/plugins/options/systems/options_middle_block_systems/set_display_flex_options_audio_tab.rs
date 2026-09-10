use crate::plugins::options::components::options_middle_block::audio_tab::OptionsUiAudioTab;
use bevy::prelude::*;

pub fn set_display_flex_options_audio_tab(
    mut visibility_of_options_audio_tab_query: Query<&mut Node, With<OptionsUiAudioTab>>,
) {
    for mut node in &mut visibility_of_options_audio_tab_query {
        node.display = Display::Grid;
    }
}
