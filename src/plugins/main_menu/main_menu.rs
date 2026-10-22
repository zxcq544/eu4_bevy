use crate::{
    core::states::GameState,
    plugins::main_menu::systems::{
        hide_continue_block::hide_continue_block, hide_main_menu_block::hide_main_menu_block,
        rotate_cube::rotate_cube_system,
        set_display_flex_main_menu_block::set_display_flex_main_menu_block,
        set_display_flex_continue_block::set_display_flex_continue_block,
    },
};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (
                set_display_flex_main_menu_block,
                set_display_flex_continue_block,
            ),
        );
        app.add_systems(OnExit(GameState::MainMenu), hide_main_menu_block);
        app.add_systems(OnExit(GameState::MainMenu), hide_continue_block);
        // Main Menu Button Checkers
        app.add_systems(
            Update,
            (rotate_cube_system).run_if(in_state(GameState::MainMenu)),
        );
    }
}
