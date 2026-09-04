use crate::{
    core::states::GameState,
    plugins::options::systems::{
        despawn_options_block::despawn_options_block,
        despawn_options_top_and_bottom_buttons::despawn_options_top_and_bottom_buttons,
        options_button_system::options_button_system, spawn_options_block::spawn_options_block,
        spawn_options_top_and_bottom_buttons::spawn_options_top_and_bottom_buttons,
    },
};
use bevy::prelude::*;

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Options),
            (spawn_options_block, spawn_options_top_and_bottom_buttons),
        );
        app.add_systems(
            OnExit(GameState::Options),
            (
                despawn_options_block,
                despawn_options_top_and_bottom_buttons,
            ),
        );

        // Options Button Checkers
        app.add_systems(
            Update,
            (options_button_system).run_if(in_state(GameState::Options)),
        );
    }
}
