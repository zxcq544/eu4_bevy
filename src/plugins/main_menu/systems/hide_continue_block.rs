use crate::plugins::main_menu::components::continue_game_entity::ContinueGameEntity;
use bevy::prelude::*;

pub fn hide_continue_block(
    mut node_of_continue_block_query: Query<&mut Node, With<ContinueGameEntity>>,
) {
    for mut node_of_continue_block in &mut node_of_continue_block_query {
        node_of_continue_block.display = Display::None;
    }
}
