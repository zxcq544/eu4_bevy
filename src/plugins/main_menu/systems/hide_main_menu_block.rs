use crate::plugins::main_menu::components::main_menu_entity::MainMenuEntity;
use bevy::prelude::*;
pub fn hide_main_menu_block(
    mut visibility_of_main_menu_block_query: Query<&mut Visibility, With<MainMenuEntity>>,
) {
    for mut visibility_of_main_menu_block in &mut visibility_of_main_menu_block_query {
        *visibility_of_main_menu_block = Visibility::Hidden;
    }
}
