use crate::{
    core::states::{GameState, OptionsTabState},
    plugins::options::{
        components::options_middle_block::audio_tab::{
            update_slider_style, update_slider_style2, update_widget_values,
        },
        systems::{
            hide_options_ui_main_entity::hide_options_ui_main_entity,
            options_button_system::options_button_system,
            options_middle_block_systems::{
                audio_volume_update_system::audio_volume_update_system,
                hide_options_audio_tab::hide_options_audio_tab,
                hide_options_video_tab::hide_options_video_tab,
                set_display_grid_options_audio_tab::set_display_grid_options_audio_tab,
                set_display_flex_options_video_tab::set_display_flex_options_video_tab,
            },
            options_top_buttons_sytems::options_top_buttons_system_united::options_top_buttons_system_united,
            set_visible_options_ui_main_entity::set_visible_options_ui_main_entity,
        },
    },
};
use bevy::prelude::*;

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Options),
            (set_visible_options_ui_main_entity,),
        );
        app.add_systems(
            OnEnter(OptionsTabState::Video),
            set_display_flex_options_video_tab,
        );
        app.add_systems(
            OnEnter(OptionsTabState::Audio),
            set_display_grid_options_audio_tab,
        );
        // app.add_systems(
        //     OnEnter(OptionsTabState::Game),
        //     set_display_flex_options_game_tab,
        // );
        // app.add_systems(
        //     OnEnter(OptionsTabState::Controls),
        //     set_display_flex_options_controls_tab,
        // );
        // We don't despawn ingame ui elements here because we want to keep them in memory
        app.add_systems(OnExit(GameState::Options), hide_options_ui_main_entity);
        app.add_systems(OnExit(OptionsTabState::Video), hide_options_video_tab);
        app.add_systems(OnExit(OptionsTabState::Audio), hide_options_audio_tab);

        // Options Button Checkers
        app.add_systems(
            Update,
            (options_button_system, options_top_buttons_system_united)
                .run_if(in_state(GameState::Options)),
        );
        // Audio tab system
        app.add_systems(
            Update,
            (
                update_widget_values,
                audio_volume_update_system.after(update_widget_values),
                update_slider_style.after(update_widget_values),
                update_slider_style2.after(update_widget_values),
            )
                .run_if(in_state(OptionsTabState::Audio)),
        );
    }
}
