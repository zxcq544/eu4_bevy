use crate::{
    core::states::GameState,
    plugins::main_menu::{
        components::main_menu_entity::{main_menu_button_action, main_menu_button_hover},
        resources::exit_delay_timer::ExitDelayTimer,
        systems::{
            free_main_menu_entity_and_resources::free_main_menu_entity_and_resources,
            handle_delayed_exit::handle_delayed_exit,
            setup_main_menu_background::setup_main_menu_background,
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
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu_background);
        app.add_systems(
            OnExit(GameState::MainMenu),
            free_main_menu_entity_and_resources,
        );
        // Button checkers
        app.add_systems(
            Update,
            (
                main_menu_button_action,
                main_menu_button_hover,
                handle_delayed_exit,
            )
                .run_if(in_state(GameState::MainMenu)),
        );
    }
}
