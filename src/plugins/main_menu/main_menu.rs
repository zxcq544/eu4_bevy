use crate::{
    core::states::GameState,
    plugins::main_menu::{
        resources::exit_delay_timer::ExitDelayTimer,
        systems::{
            free_main_menu_entity_and_resources::free_main_menu_entity_and_resources,
            handle_delayed_exit::handle_delayed_exit,
            // main_menu_button_actions::main_menu_button_actions,
            // main_menu_button_hover::main_menu_button_hover,
            // main_menu_button_sounds::main_menu_button_sounds,
            main_menu_button_system_united::main_menu_button_system_united,
            rotate_cube::rotate_cube_system,
            spawn_continue_block::spawn_continue_block,
            spawn_main_menu_scene_with_cam::spawn_main_menu_scene_with_cam,
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
            (spawn_main_menu_scene_with_cam, spawn_continue_block),
        );
        app.add_systems(
            OnExit(GameState::MainMenu),
            free_main_menu_entity_and_resources,
        );
        // Button checkers
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
