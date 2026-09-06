use crate::{
    core::states::GameState,
    plugins::options::systems::{
        hide_options_block::hide_options_block,
        hide_options_block_top_and_bottom_buttons::hide_options_block_top_and_bottom_buttons,
        options_button_system::options_button_system,
        set_visible_options_block::set_visible_options_block,
        set_visible_options_buttons_top_and_buttom_rows::set_visible_options_block_top_and_bottom_buttons,
    },
};
use bevy::prelude::*;

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Options),
            (
                set_visible_options_block,
                set_visible_options_block_top_and_bottom_buttons,
            ),
        );
        // We don't despawn ingame ui elements here because we want to keep them in memory
        app.add_systems(
            OnExit(GameState::Options),
            (
                hide_options_block,
                hide_options_block_top_and_bottom_buttons,
            ),
        );

        // Options Button Checkers
        app.add_systems(
            Update,
            (options_button_system).run_if(in_state(GameState::Options)),
        );
    }
}
