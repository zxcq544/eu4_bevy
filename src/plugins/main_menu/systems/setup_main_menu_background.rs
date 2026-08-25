use crate::plugins::main_menu::{
    components::main_menu_entity::MainMenuEntity,
    resources::{
        background_image_for_continue::BackgroundImageForContinue,
        background_image_of_main_menu::BackgroundImageOfMainMenu,
        bg_image_lower_panel_main_menu_left_button::BgImageLowerPanelMainMenuLeftButton,
        main_menu_multiplayer_button_image::MainMenuMultiplayerButtonImage,
        main_menu_single_player_button_image::MainMenuSinglePlayerButtonImage,
    },
};
use bevy::prelude::*;

pub fn setup_main_menu_background(
    main_menu_background_image: Res<BackgroundImageOfMainMenu>,
    continue_background_image: Res<BackgroundImageForContinue>,
    single_player_button_image: Res<MainMenuSinglePlayerButtonImage>,
    multiplayer_button_image: Res<MainMenuMultiplayerButtonImage>,
    bg_image_lower_panel_main_menu_left_button: Res<BgImageLowerPanelMainMenuLeftButton>,
    mut commands: Commands,
) {
    info!("Setting up main menu background");
    commands.spawn((Camera2d::default(), MainMenuEntity));
    commands.spawn_scene_list(MainMenuEntity::as_scene_list(
        main_menu_background_image,
        continue_background_image,
        single_player_button_image,
        multiplayer_button_image,
        bg_image_lower_panel_main_menu_left_button,
    ));
}
