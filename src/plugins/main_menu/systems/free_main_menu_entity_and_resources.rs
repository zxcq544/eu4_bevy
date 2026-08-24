use crate::plugins::main_menu::{
    components::main_menu_entity::MainMenuEntity,
    resources::{
        background_image_for_continue::BackgroundImageForContinue,
        background_image_of_main_menu::BackgroundImageOfMainMenu,
        main_menu_multiplayer_button_image::MainMenuMultiplayerButtonImage,
        main_menu_single_player_button_image::MainMenuSinglePlayerButtonImage,
    },
};
use bevy::prelude::*;

pub fn free_main_menu_entity_and_resources(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuEntity>>,
) {
    info!("Freeing main menu entity and resources");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<BackgroundImageOfMainMenu>();
    commands.remove_resource::<BackgroundImageForContinue>();
    commands.remove_resource::<MainMenuSinglePlayerButtonImage>();
    commands.remove_resource::<MainMenuMultiplayerButtonImage>();
}
