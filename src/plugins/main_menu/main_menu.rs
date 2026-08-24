use crate::{
    core::states::GameState,
    plugins::main_menu::systems::{
        free_main_menu_entity_and_resources::free_main_menu_entity_and_resources,
        setup_main_menu_background::setup_main_menu_background,
    },
};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu_background);
        app.add_systems(
            OnExit(GameState::MainMenu),
            free_main_menu_entity_and_resources,
        );
    }
}
