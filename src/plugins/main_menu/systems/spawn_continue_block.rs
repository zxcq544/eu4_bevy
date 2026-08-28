use crate::plugins::main_menu::{
    components::continue_game_entity::ContinueGameEntity,
    resources::main_menu_all_images::MainMenuAllImages,
};
use bevy::prelude::*;
use bevy_fluent::Localization;
use fonts::FontHandles;

pub fn spawn_continue_block(
    mut commands: Commands,
    fonts: Res<FontHandles>,
    localization_res: Res<Localization>,
    main_menu_all_images_res: Res<MainMenuAllImages>,
) {
    let show_continue_button = true;
    if show_continue_button {
        commands.spawn_scene_list(ContinueGameEntity::as_scene_list(
            &main_menu_all_images_res,
            &localization_res,
            &fonts,
        ));
    }
}
