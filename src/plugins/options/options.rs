use crate::{
    core::states::{GameState, OptionsTab},
    plugins::options::systems::{
        hide_options_ui_main_entity::hide_options_ui_main_entity,
        hide_options_video_tab::hide_options_video_tab,
        options_button_system::options_button_system,
        set_visible_options_ui_main_entity::set_visible_options_ui_main_entity,
        set_visible_options_video_tab::set_visible_options_video_tab,
    },
};
use bevy::prelude::*;

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Options),
            (
                set_visible_options_ui_main_entity,
                set_visible_options_video_tab, // move this when video tab active
            ),
        );
        // We don't despawn ingame ui elements here because we want to keep them in memory
        app.add_systems(OnExit(GameState::Options), (hide_options_ui_main_entity,));
        app.add_systems(OnExit(OptionsTab::Video), hide_options_video_tab);

        // Options Button Checkers
        app.add_systems(
            Update,
            (options_button_system).run_if(in_state(GameState::Options)),
        );
    }
}
