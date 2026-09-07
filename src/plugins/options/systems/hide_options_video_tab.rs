use crate::plugins::options::components::options_video_tab_entity::OptionsVideoTabEntity;
use bevy::prelude::*;

pub fn hide_options_video_tab(
    mut visibility_of_options_video_tab_query: Query<&mut Visibility, With<OptionsVideoTabEntity>>,
) {
    for mut visibility_of_options_video_tab in &mut visibility_of_options_video_tab_query {
        *visibility_of_options_video_tab = Visibility::Hidden;
    }
}
