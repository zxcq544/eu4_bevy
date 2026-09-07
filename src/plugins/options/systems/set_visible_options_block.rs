use crate::plugins::options::components::options_background_entity::OptionsBackgroundEntity;
use bevy::prelude::*;

pub fn set_visible_options_block(
    mut visibility_of_options_blocks_query: Query<&mut Visibility, With<OptionsBackgroundEntity>>,
) {
    for mut visibility_of_options_block in &mut visibility_of_options_blocks_query {
        *visibility_of_options_block = Visibility::Visible;
    }
}
