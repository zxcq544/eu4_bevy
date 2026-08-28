use crate::plugins::main_menu::{
    components::continue_game_entity::ContinueGameEntity,
    resources::main_menu_all_images::MainMenuAllImages,
};
use bevy::prelude::*;
use bevy_fluent::Localization;
use fonts::FontHandles;
use settings::Settings;

pub fn spawn_continue_block(
    mut commands: Commands,
    fonts: Res<FontHandles>,
    localization_res: Res<Localization>,
    main_menu_all_images_res: Res<MainMenuAllImages>,
    settings: Res<Settings>,
) {
    let last_saved_game_exists = settings.last_saved_game.last_save_game_exists;
    if last_saved_game_exists {
        commands.spawn_scene_list(ContinueGameEntity::as_scene_list(
            &main_menu_all_images_res,
            &localization_res,
            &fonts,
        ));
    }
}
