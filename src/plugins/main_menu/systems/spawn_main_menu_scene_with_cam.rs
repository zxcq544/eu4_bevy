use crate::plugins::main_menu::{
    components::main_menu_entity::MainMenuEntity,
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
    info!("Setting up main menu background");
    commands.spawn((Camera2d::default(), MainMenuEntity));
    commands.spawn_scene_list(MainMenuEntity::as_scene_list(
        main_menu_all_images_res,
        &localization_res,
        fonts,
    ));
}
