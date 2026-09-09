use crate::plugins::main_menu::components::continue_game_entity::ContinueGameEntity;
use bevy::prelude::*;

pub fn hide_continue_block(
    mut visibility_of_continue_block_query: Query<&mut Visibility, With<ContinueGameEntity>>,
) {
    for mut visibility_of_continue_block in &mut visibility_of_continue_block_query {
        *visibility_of_continue_block = Visibility::Hidden;
    }
}
