use crate::{
    core::states::GameState,
    plugins::{
        main_menu::{
            resources::exit_delay_timer::ExitDelayTimer,
            systems::{
                despawn_continue_block::despawn_continue_block,
                despawn_main_menu_entity::despawn_main_menu_entity,
                handle_delayed_exit::handle_delayed_exit,
                main_menu_button_system_united::main_menu_button_system_united,
                rotate_cube::rotate_cube_system, spawn_continue_block::spawn_continue_block,
                spawn_main_menu_scene::spawn_main_menu_scene,
            },
        },
        options::systems::{
            spawn_options_block::spawn_options_block,
            spawn_options_top_and_bottom_buttons::spawn_options_top_and_bottom_buttons,
            spawn_options_video_tab::spawn_options_video_tab,
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
            (
                spawn_main_menu_scene,
                spawn_continue_block,
                // Spawn options block here as hidden so we don't spawn despawn often
                spawn_options_block,
                spawn_options_top_and_bottom_buttons,
                spawn_options_video_tab,
            ),
        );
        app.add_systems(OnExit(GameState::MainMenu), despawn_main_menu_entity);
        app.add_systems(OnExit(GameState::MainMenu), despawn_continue_block);
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
