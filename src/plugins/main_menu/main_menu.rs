use crate::{
    core::states::GameState,
    plugins::main_menu::{
        resources::exit_delay_timer::ExitDelayTimer,
        systems::{
            handle_delayed_exit::handle_delayed_exit, hide_continue_block::hide_continue_block,
            hide_main_menu_block::hide_main_menu_block,
            main_menu_button_system_united::main_menu_button_system_united,
            rotate_cube::rotate_cube_system,
            set_visible_continue_block::set_visible_continue_block,
            set_visible_main_menu_block::set_visible_main_menu_block,
        },
    },
};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ExitDelayTimer {
            timer: Timer::from_seconds(0.0, TimerMode::Once),
            should_exit: false,
        });
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (set_visible_main_menu_block, set_visible_continue_block),
        );
        app.add_systems(OnExit(GameState::MainMenu), hide_main_menu_block);
        app.add_systems(OnExit(GameState::MainMenu), hide_continue_block);
        // Main Menu Button Checkers
        app.add_systems(
            Update,
            (
                // main_menu_button_actions,
                // main_menu_button_hover,
                // main_menu_button_sounds,
                main_menu_button_system_united,
                handle_delayed_exit,
                rotate_cube_system,
            )
                .run_if(in_state(GameState::MainMenu)),
        );
    }
}
