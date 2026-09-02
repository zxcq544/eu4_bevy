use crate::plugins::main_menu::{
    components::{
        continue_game_entity::ContinueGameEntity, main_menu_entity::MainMenuEntity,
        options_entity::OptionsEntity,
    },
    resources::options_images::OptionsImages,
};
use bevy::prelude::*;
use bevy_fluent::Localization;
use fonts::FontHandles;

pub fn spawn_options_block(
    mut commands: Commands,
    fonts: Res<FontHandles>,
    localization_res: Res<Localization>,
    options_images: Res<OptionsImages>,
    mut query: Query<&mut Visibility, Or<(With<MainMenuEntity>, With<ContinueGameEntity>)>>,
) {
    info!("Spawning options block");
    commands.spawn_scene_list(OptionsEntity::as_scene_list(
        &options_images,
        &localization_res,
        &fonts,
    ));
    // hide main menu block and continue game block
    for mut visibility in &mut query {
        *visibility = Visibility::Hidden;
    }
}

// fn hide_main_menu_block(
//     commands: &mut Commands,
//     mut query: Query<&mut Visibility, With<MainMenuEntity>>,
// ) {
//     for mut visibility in &mut query {
//         *visibility = Visibility::Hidden;
//     }
// }
