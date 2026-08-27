use crate::plugins::main_menu::{
    components::{continue_game_entity::ContinueGameEntity, main_menu_entity::MainMenuEntity},
    resources::main_menu_all_images::MainMenuAllImages,
};
use bevy::prelude::*;
use bevy_fluent::Localization;
use fonts::FontHandles;

pub fn spawn_main_menu_scene_with_cam(
    main_menu_all_images_res: Res<MainMenuAllImages>,
    mut commands: Commands,
    localization_res: Res<Localization>,
    fonts: Res<FontHandles>,
) {
    let show_continue_button = true;
    info!("Setting up main menu background");
    commands.spawn((Camera2d::default(), MainMenuEntity));
    commands.spawn_scene_list(MainMenuEntity::as_scene_list(
        &main_menu_all_images_res,
        &localization_res,
        &fonts,
    ));
    if show_continue_button {
        commands.spawn_scene_list(ContinueGameEntity::as_scene_list(
            &main_menu_all_images_res,
            &localization_res,
            &fonts,
        ));
    }
}
