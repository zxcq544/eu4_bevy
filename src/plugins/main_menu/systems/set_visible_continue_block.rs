use crate::plugins::main_menu::components::continue_game_entity::ContinueGameEntity;
use bevy::prelude::*;
use settings::Settings;

pub fn set_visible_continue_block(
    settings: Res<Settings>,
    mut visibility_of_continue_block_query: Query<&mut Visibility, With<ContinueGameEntity>>,
) {
    let last_saved_game_exists = settings.last_saved_game.last_save_game_exists;
    if last_saved_game_exists {
        for mut visibility_of_continue_block in &mut visibility_of_continue_block_query {
            *visibility_of_continue_block = Visibility::Visible;
        }
    }
}
