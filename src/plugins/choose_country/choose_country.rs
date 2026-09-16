use crate::{
    core::states::GameState,
    plugins::choose_country::systems::{
        bottom_left_block_button_system_for_choose_country::bottom_left_block_button_system_for_choose_country,
        hide_choose_country_ui_main_entity::hide_choose_country_ui_main_entity,
        set_display_flex_for_choose_country_ui_main_entity::set_display_flex_for_choose_country_ui_main_entity,
    },
};
use bevy::prelude::*;
pub struct ChooseCountryPlugin;

impl Plugin for ChooseCountryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::ChooseCountry),
            set_display_flex_for_choose_country_ui_main_entity,
        );
        app.add_systems(
            Update,
            (bottom_left_block_button_system_for_choose_country)
                .run_if(in_state(GameState::ChooseCountry)),
        );
        app.add_systems(
            OnExit(GameState::ChooseCountry),
            hide_choose_country_ui_main_entity,
        );
    }
}
