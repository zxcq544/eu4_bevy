use crate::plugins::main_menu::components::main_menu_entity::MainMenuEntity;
use bevy::prelude::*;

pub fn set_display_flex_main_menu_block(
    mut node_of_main_menu_block_query: Query<&mut Node, With<MainMenuEntity>>,
) {
    for mut node_of_main_menu_block in &mut node_of_main_menu_block_query {
        node_of_main_menu_block.display = Display::Flex;
    }
}
