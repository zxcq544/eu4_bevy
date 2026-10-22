use crate::{
    plugins::main_menu::components::continue_game_entity::ContinueGameEntity,
    shared::settings::settings::Settings,
};
use bevy::prelude::*;

pub fn set_display_flex_continue_block(
    settings: Res<Settings>,
    mut node_of_continue_block_query: Query<&mut Node, With<ContinueGameEntity>>,
) {
    let last_saved_game_exists = settings.last_saved_game.last_save_game_exists;
    if last_saved_game_exists {
        for mut node_of_continue_block in &mut node_of_continue_block_query {
            node_of_continue_block.display = Display::Flex;
        }
    }
}
